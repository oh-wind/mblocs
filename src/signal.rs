use std::{sync::{Mutex, mpsc::{Sender, Receiver}}, collections::HashMap};

use libc::{ sigaction, siginfo_t, SA_SIGINFO };

lazy_static!{
    static ref SIGNAL_SENDERS: Mutex<Signal> = {
       Mutex::new( Signal::new() )
    };
}

extern "C" fn signal_handler(signal: i32, info: *mut siginfo_t, _: *mut libc::c_void){ unsafe {
    let sig = SIGNAL_SENDERS.lock().unwrap();
    if let Some(senders) = sig.registers.get(&signal){
        for sender in senders {
            sender.send((signal, (*info).si_code)).unwrap();
        }
    }
}}


pub struct Signal{
    registers: HashMap<i32, Vec<Sender<(i32, i32)>>>
}


impl Signal {
    pub fn new() -> Self {
        Self{ registers: HashMap::new() }
    }


    /// Return a receiver, which will generate  data when any given signal is triggered.
    pub fn reg(signal: Vec<i32>) -> Receiver<(i32, i32)> {

        let mut action = unsafe { std::mem::zeroed::<sigaction>() };
        action.sa_sigaction = signal_handler   as usize;
        action.sa_flags = SA_SIGINFO;
        let (tx, rx) = std::sync::mpsc::channel();
        let mut ss = SIGNAL_SENDERS.lock().unwrap();
        for i in signal {
            if  ss.registers.get(&i).is_none() {
                unsafe {
                    sigaction(i, &action, std::ptr::null_mut());
                }
                ss.registers.insert(i, Vec::new());
            }

            let senders = ss.registers.get_mut(&i).unwrap();
            senders.push(tx.clone());
        }
        rx
    }
}
