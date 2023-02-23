use crate::block::Block;
#[allow(unused_imports)]
use crate::block::BlockType::{Interval, Once, Signal};
#[allow(unused_imports)]
use crate::block::CommandType::{Function, Shell};
use crate::blocks::clicksignal;

use super::block::Env;
use crate::blocks::batterstate::{self, battery_status};
use crate::blocks::cpu::cpu_usage;
use crate::blocks::datetime::current_time;
use crate::blocks::memory::memory_usage;
use crate::blocks::clicksignal::clicksignal;
use std::error::Error;
use std::fmt::Display;

pub const SEPARATOR: &str = " | ";
pub const PREFIX: &str = " ";
pub const SUFFIX: &str = " ";

pub const CHARGIN_ICON: &'static str = "\u{f0084}";
pub const AC_LINKED_ICON: &'static str = "\u{f1211}";

// 󰁺 󰁻 󰁼 󰁽 󰁾 󰁿 󰂀 󰂁 󰂂 󰁹
pub const BT_ICONS: [&'static str; 10] = [
    "\u{f007a}", //10
    "\u{f007b}", //20
    "\u{f007c}", //30
    "\u{f007d}", //40
    "\u{f007e}", //50
    "\u{f007f}", //60
    "\u{f0080}", //70
    "\u{f0081}", //80
    "\u{f0082}", //90
    "\u{f0079}", //100
];

pub const BAT_PATH: &'static str = "/sys/class/power_supply/BAT1";
pub const AC_PATH: &'static str = "/sys/class/power_supply/ACAD";

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
        kind: Once,
        command: Function(clicksignal),
        prefix: "",
        suffix: "",
        signal: Some(1),
    },
    Block {
        kind: Once,
        command: Function(clicksignal),
        prefix: "",
        suffix: "",
        signal: Some(2),
    },
    Block {
        kind: Once,
        command: Function(clicksignal),
        prefix: "",
        suffix: "",
        signal: Some(3),
    },
];
