use crate::config;
use std::error::Error;
use std::fmt::Display;
use std::process::Command;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum BlockType {
    Once,
    Interval(u64),
    Signal,
}
#[derive(Debug, Clone, Copy)]
pub struct Env {
    pub signal:  i32,
    pub sigcomp: i32,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum CommandType<'a> {
    Shell(&'a [&'a str]),
    Function(fn(Option<Env>) -> Result<Box<dyn Display>, Box<dyn Error>>),
}

#[derive(Debug)]
pub struct Block<'a> {
    pub kind: BlockType,
    pub command: CommandType<'a>,
    pub prefix: &'a str,
    pub suffix: &'a str,
    pub signal: Option<i32>,
}

impl Block<'_> {
    pub fn execute(&self, env: Option<Env>) -> Option<String> {
        if let Some(e) = env {
            println!("{:?}, {:?}", self, e);
        }
        match self.command {
            CommandType::Shell(cmd) => {
                let l: usize = cmd.len();
                if l == 0 {
                    return None;
                }
                let mut command = Command::new(cmd[0]);
                if l > 1 {
                    command.args(&cmd[1..]);
                }
                let output;
                if let Ok(r) = command.output() {
                    output = r;
                } else {
                    return None;
                }
                if !output.status.success() {
                    return None;
                }
                match String::from_utf8(output.stdout) {
                    Ok(s) => {
                        if s.is_empty() {
                            Some(s)
                        } else {
                            Some(concat_string!(self.prefix, s.trim(), self.suffix))
                        }
                    }
                    Err(_) => None,
                }
            }
            CommandType::Function(func) => match func(env) {
                Ok(r) => {
                    let s = r.to_string();
                    if s.is_empty() {
                        Some(s)
                    } else {
                        Some(concat_string!(self.prefix, s, self.suffix))
                    }
                }
                Err(_) => None,
            },
        }
    }
}

pub fn infer_status(outputs: &[String]) -> String {
    let rootname = outputs
        .iter()
        .filter_map(|e| {
            if !(*e).is_empty() {
                Some(e.to_owned())
            } else {
                None
            }
        })
        .collect::<Vec<String>>()
        .join(config::SEPARATOR);
    concat_string!(config::PREFIX, rootname, config::SUFFIX)
}
