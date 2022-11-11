use std::process::Command;
use std::str;

use serde::{Deserialize};


#[derive(Deserialize, Clone)]
pub struct Display {
    pub name: String,
    pub active: bool,
    pub modes: Vec<Mode>,
    pub current_mode: Option<Mode>
}

#[derive(Deserialize, Clone)]
pub struct Mode {
    pub width: u32,
    pub height: u32,
    pub refresh: u32
}

pub fn get_display_modes() -> Vec<(Display, Mode)> {
    let displays = get_displays();
    let mut display_modes = Vec::new();
    for display in displays {
        for mode in &display.modes {
            display_modes.push( (display.clone(), mode.clone()) )
        }
    }
    display_modes
}

pub fn set_output(display_mode: (Display, Mode)) {
    Command::new("swaymsg")
        .arg("output")
        .arg(display_mode.0.name)
        .arg("pos")
        .arg("0")
        .arg("0")
        .arg("res")
        .arg(format!("{}x{}", display_mode.1.width, display_mode.1.height))
        .output()
        .expect("failed to execute process");
}

pub fn get_displays() -> Vec<Display> {
    let output = Command::new("swaymsg")
        .arg("-t")
        .arg("get_outputs")
        .arg("-r")
        .output()
        .expect("failed to execute process")
        .stdout;
    let output_str = str::from_utf8(&output).unwrap();
    serde_json::from_str(output_str).unwrap()
}
