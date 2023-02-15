mod linux;
mod windows;

use log::warn;

const OS: &str = std::env::consts::OS;

pub fn start_home_assistant() {
    match OS {
        "linux" => linux::ha(),
        _ => warn!("start_home_assistant: not implemented for {}", OS),
    }
}
