use serde::{Deserialize, Serialize};
use std::error;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ConfigAndStatus {
    pub config: Option<Config>,
    pub status: Status,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub sets: Option<u32>,
    pub work: Option<u64>,
    pub short_break: Option<u64>,
    pub long_break: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    pub current_set: u32,
}

impl Default for Status {
    fn default() -> Self {
        Self { current_set: 1 }
    }
}

impl ConfigAndStatus {
    fn write_to_disk(&self) -> Result<(), std::io::Error> {
        let mut toml_file = std::env::temp_dir();
        toml_file.push("pomodoro_timer.toml");

        let config_pretty_string = toml::to_string_pretty(self).unwrap();
        std::fs::write(toml_file, config_pretty_string)
    }

    pub fn get_sets(&self) -> Option<u32> {
        self.config.as_ref().and_then(|config| config.sets)
    }

    pub fn get_short_break_time(&self) -> Option<u64> {
        self.config.as_ref().and_then(|config| config.short_break)
    }

    pub fn get_long_break_time(&self) -> Option<u64> {
        self.config.as_ref().and_then(|config| config.long_break)
    }

    pub fn get_work_time(&self) -> Option<u64> {
        self.config.as_ref().and_then(|config| config.work)
    }
}

pub fn get_state() -> Option<ConfigAndStatus> {
    let mut toml_file = std::env::temp_dir();
    toml_file.push("pomodoro_timer.toml");
    let contents = std::fs::read_to_string(toml_file).ok()?;
    toml::from_str::<ConfigAndStatus>(&contents).ok()
}

pub fn write_default() -> Result<(), std::io::Error> {
    let config_and_status = ConfigAndStatus::default();
    config_and_status.write_to_disk()
}

pub fn increment_set() -> Result<(), std::io::Error> {
    let mut state = get_state().unwrap_or_default();
    state.status.current_set += 1;
    state.write_to_disk()
}

pub fn reset_set() -> Result<(), std::io::Error> {
    let mut state = get_state().unwrap_or_default();
    state.status.current_set = 1;
    state.write_to_disk()
}
