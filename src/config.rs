use crate::block::Block;
#[allow(unused_imports)]
use crate::block::BlockType::{Interval, Once, Signal};
#[allow(unused_imports)]
use crate::block::CommandType::{Function, Shell};

use crate::blocks::batterstate::{self, battery_status};
use crate::blocks::cpu::cpu_usage;
use crate::blocks::datetime::current_time;
use crate::blocks::memory::memory_usage;
use std::fmt::Display;
use std::error::Error;
use super::block::Env;

pub const SEPARATOR: &str = " | ";
pub const PREFIX: &str = " ";
pub const SUFFIX: &str = " ";

pub const CHARGIN_ICON: &'static str = "\u{f0084}";
pub const AC_LINKED_ICON: &'static str = "\u{f1211}";

// 󰁺 󰁻 󰁼 󰁽 󰁾 󰁿 󰂀 󰂁 󰂂 󰁹 
pub const BT_ICONS: [&'static str; 10] = ["\u{f007a}", "\u{f007b}", "\u{f007c}", "\u{f007d}", "\u{f007e}", "\u{f007f}", "\u{f0080}", "\u{f0081}", "\u{f0082}", "\u{f0079}", ];

pub const BAT_PATH: &'static str = "/sys/class/power_supply/BAT1";
pub const AC_PATH: &'static str = "/sys/class/power_supply/ACAD";

pub fn testsss( env: Option<Env>) -> Result<Box<dyn Display>, Box<dyn Error>>{
    println!("test: {:?}", env);
    return Ok(Box::new(format!("{:?}", env)));
}
pub const BLOCKS: &[Block] = &[
    Block {
        kind: Interval(1),
        command: Function(cpu_usage),
        prefix: "\u{f85a}: ", //icon
        suffix: "%",
        signal: None,
    },
    Block {
        kind: Interval(1),
        command: Function(memory_usage),
        prefix: "",
        suffix: "",
        signal: None,
    },
    Block {
        kind: Interval(1800),
        command: Shell(&["date", "+%Y/%m/%d"]),
        prefix: "",
        suffix: "",
        signal: None,
    },
    Block {
        kind: Interval(30),
        command: Function(current_time),
        prefix: "",
        suffix: "",
        signal: None,
    },
    Block {
        kind: Interval(2),
        command: Function(battery_status),
        prefix: "",
        suffix: "",
        signal: None,
    },
    Block {
        kind: Once,
        command: Shell(&["whoami"]),
        prefix: "",
        suffix: "",
        signal: None,
    },
    Block {
        kind: Signal,
        command: Function(testsss),
        prefix: "",
        suffix: "",
        signal: Some(1),
    }
];
