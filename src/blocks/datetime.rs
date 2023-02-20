use chrono::Local;
use std::error::Error;
use std::fmt::Display;

use crate::block::Env;

pub fn current_time(_: Option<Env>) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let t = Local::now().format("%I:%M %p");
    Ok(Box::new(t))
}

pub fn current_date(_: Option<Env>) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let d = Local::now().format("%a, %b %d %Y");
    Ok(Box::new(d))
}
