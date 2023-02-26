use crate::Env;
use std::fmt::Display;
use std::error::Error;


pub fn clicksignal(env: Option<Env>) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(Box::new(""))
}
