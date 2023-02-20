use psutil::cpu::CpuPercentCollector;
use std::error::Error;
use std::fmt::Display;
use std::thread::sleep;
use std::time::Duration;

use crate::block::Env;

pub fn cpu_usage(env: Option<Env>) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let mut collector = CpuPercentCollector::new()?;
    sleep(Duration::from_millis(750));
    let usage = collector.cpu_percent()?.round() as u64;
    Ok(Box::new(usage))
}
