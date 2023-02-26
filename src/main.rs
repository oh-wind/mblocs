#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate concat_string;

use block::Env;
use libc::{SIGHUP, SIGINT, SIGTERM};
use std::{
    ffi::CString,
    ptr,
    sync::mpsc::{channel, Receiver, Sender},
    thread,
    time::Duration,
};
use x11::xlib;

mod block;
mod blocks;
mod config;
mod signal;

use libc::SIGRTMIN;


/// await given signum and [`SIGTERM`, `SIGHUP`, `SIGINT`], then send the received signal by tx.
/// if the signum is -1, only await the terminal signals.
fn await_signals(tx: Sender<(i32, i32)>, signum: i32) {
    let rev = if signum == -1 {
        signal::Signal::reg([SIGTERM, SIGHUP, SIGINT].to_vec())
    } else if signum > 0 {
        signal::Signal::reg([signum, SIGTERM, SIGINT, SIGHUP].to_vec())
    } else {
        return;
    };
    loop { 
        let sig = rev.recv().unwrap();
        tx.send(sig).unwrap();
        if is_terminal(sig.0) {
            break;
        }
    }
}

fn is_terminal(sig: i32) -> bool {
    sig == SIGTERM || sig == SIGINT || sig == SIGHUP
}

fn main() {
    let (tx, rx): (
        Sender<(usize, Option<String>)>, // index, msg, signal
        Receiver<(usize, Option<String>)>,
    ) = channel();
    let mut handles = vec![];
    let mut outputs = vec![String::from(""); config::BLOCKS.len()];
    let display = unsafe { xlib::XOpenDisplay(ptr::null()) };
    let window = unsafe { xlib::XDefaultRootWindow(display) };

    // fire threads
    for (i, b) in config::BLOCKS.iter().enumerate() {
        let tx_clone = tx.clone();
        let (tx_signals, rx_signals): (Sender<(i32, i32)>, Receiver<(i32, i32)>) = channel();

        let sig = if let Some(s) = b.signal { s } else { -1 };
        if sig == -1 || (sig >= 0 && sig < 15) {
            handles.push(thread::spawn(move || {
                await_signals(tx_signals, SIGRTMIN() + sig);
            }));
        } else {
            println!("Warning: not support signal: {}", sig);
            println!("The block has been skipped.");
            continue;
        }

        match b.kind {
            block::BlockType::Once => handles.push(thread::spawn(move || {
                let msg = b.execute(None);
                tx_clone.send((i, msg)).unwrap();
                while let Ok((signal, sigcomp)) = rx_signals.recv() {
                    match signal {
                        SIGTERM | SIGINT | SIGHUP => {
                            tx_clone.send((usize::MAX, None)).unwrap();
                            break;
                        }
                        _signum => {
                            let msg = b.execute(Some(Env { signal: _signum, sigcomp }));
                            tx_clone.send((i, msg)).unwrap();
                        }
                    }
                }
            })),
            block::BlockType::Interval(t) => {
                let handle = thread::spawn(move || {
                    let msg = b.execute(None);
                    let tx_clone = tx_clone;
                    tx_clone.send((i, msg)).unwrap();
                    loop {
                        if let Ok((signal, sigcomp)) =
                            rx_signals.recv_timeout(Duration::from_secs(t))
                        {
                            if is_terminal(signal) {
                                tx_clone.send((usize::MAX, None)).unwrap();
                                break;
                            } else {
                                let msg = b.execute(Some(Env { signal, sigcomp }));
                                tx_clone.send((i, msg)).unwrap();
                            }
                        } else {
                            let msg = b.execute(None);
                            tx_clone.send((i, msg)).unwrap();
                        }
                    }
                });
                handles.push(handle);
            }
            block::BlockType::Signal => {
                let s = b.signal.unwrap();
                if s < 1 || s > 15 {
                    tx_clone.send((i, None)).unwrap();
                    continue;
                }
                let msg = b.execute(None);
                tx_clone.send((i, msg)).unwrap();
                let handle = thread::spawn(move || {
                    while let Ok((signal, sigcomp)) = rx_signals.recv() {
                        match signal {
                            SIGTERM | SIGINT | SIGHUP => {
                                tx_clone.send((usize::MAX, None)).unwrap();
                                break;
                            }
                            _signum => {
                                let msg = b.execute(Some(Env { signal: _signum, sigcomp }));
                                tx_clone.send((i, msg)).unwrap();
                            }
                        }
                    }
                });
                handles.push(handle);
            }
        }
    }

    // update status if a block change occurs
    drop(tx);
    while let Ok((i, msg)) = rx.recv() {
        if i == usize::MAX {
            break;
        }
        let msg = match msg {
            Some(s) => s,
            None => "failed".to_string(),
        };
        if outputs[i] == msg {
            continue;
        }
        outputs[i] = msg;
        let status = block::infer_status(&outputs);
        let c_str = CString::new(status.as_str()).expect("panic caused by cstring");
        let str_ptr = c_str.as_ptr() as *const i8;
        unsafe {
            xlib::XStoreName(display, window, str_ptr);
            xlib::XSync(display, 0);
        }
    }

    // graceful termination of threads
    for handle in handles {
        let _ = handle.join();
    }

    // cleanup of the status and close the open display
    let c_str = CString::new("").unwrap();
    let str_ptr = c_str.as_ptr() as *const i8;
    unsafe {
        xlib::XStoreName(display, window, str_ptr);
        xlib::XSync(display, 0);
        xlib::XCloseDisplay(display);
    }
}
