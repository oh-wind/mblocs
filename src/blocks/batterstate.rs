use crate::block::Env;
use std::error::Error;
use std::fmt::Display;
use std::fs::read_to_string;
use std::path::PathBuf;

use crate::config::*;

static mut SHOULD_FLICK: bool = false;

pub fn battery_status(_: Option<Env>) -> Result<Box<dyn Display>, Box<dyn Error>> {
    let bat_state_path: PathBuf = [BAT_PATH, "status"].iter().collect();
    let now_eng_path: PathBuf = [BAT_PATH, "energy_now"].iter().collect();
    let full_eng_path: PathBuf = [BAT_PATH, "energy_full"].iter().collect();
    let ac_online: PathBuf = [AC_PATH, "online"].into_iter().collect();

    let status = read_to_string(&bat_state_path)?;

    if status.trim_end() == "Charging".to_string() {
        return Ok(Box::new(CHARGIN_ICON));
    }

    let eng_now_s = read_to_string(&now_eng_path)?.trim_end().to_string();
    let eng_full_s = read_to_string(&full_eng_path)?.trim_end().to_string();

    let eng_now: i32 = eng_now_s.parse().unwrap_or_default();
    let eng_full: i32 = eng_full_s.parse().unwrap_or_default();

    if eng_full <= 0 || eng_now < 0 {
        return Ok(Box::new("\u{f0083} parse error"));
    }

    let pl = (eng_now as f32 / eng_full as f32) * 10.;
    if pl as usize == 0 {
        return Ok(Box::new(BT_ICONS[0]));
    }
    if let Ok(is_ac_link) = read_to_string(ac_online) {
        if is_ac_link.trim() == "1" {
            unsafe {
                if SHOULD_FLICK {
                    SHOULD_FLICK = false;
                    return Ok(Box::new(AC_LINKED_ICON));
                } else {
                    SHOULD_FLICK = true;
                }
            }
        }
    }
    return Ok(Box::new(BT_ICONS[(pl as usize) - 1]));
}
