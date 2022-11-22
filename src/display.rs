use std::process::Command;
use std::str;

use serde::Deserialize;

use std::collections::HashMap;

#[derive(Deserialize, Clone, Debug)]
pub struct Display {
    pub name: String,
    pub active: bool,
    pub modes: Vec<Mode>,
    pub current_mode: Option<Mode>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Mode {
    pub width: u32,
    pub height: u32,
    pub refresh: u32,
}

impl Display {
    pub fn process(&self) {
        self.onoff(self.active);
        match &self.current_mode {
            Some(mode) => self.set_output(mode.clone()),
            None => (),
        }
    }

    pub fn set_output(&self, display_mode: Mode) {
        Command::new("swaymsg")
            .arg("output")
            .arg(&self.name)
            .arg("mode")
            .arg(format!(
                "{}x{}@{}Hz",
                display_mode.width,
                display_mode.height,
                display_mode.refresh as f32 / 1000_f32
            ))
            .output()
            .expect("failed to execute process");
    }

    pub fn onoff(&self, on: bool) {
        let command = match on {
            true => "enable",
            false => "disable",
        };
        Command::new("swaymsg")
            .arg("output")
            .arg(&self.name)
            .arg(command)
            .output()
            .expect("failed to execute process");
    }

    pub fn is_current_mode(&self, mode: Mode) -> bool {
        match &self.current_mode {
            Some(current_mode) => {
                current_mode.height == mode.height
                    && mode.width == current_mode.width
                    && mode.refresh == current_mode.refresh
            }
            None => false,
        }
    }
}

pub fn get_displays() -> HashMap<String, Display> {
    let output = Command::new("swaymsg")
        .arg("-t")
        .arg("get_outputs")
        .arg("-r")
        .output()
        .expect("failed to execute process")
        .stdout;
    let output_str = str::from_utf8(&output).unwrap();
    let displays: Vec<Display> = serde_json::from_str(output_str).unwrap();
    displays.into_iter().map(|d| (d.name.clone(), d)).collect()
}
