use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;

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
    fn write_to_disk(&self) -> Result<()> {
        let mut config_path = std::env::temp_dir();
        config_path.push("pomodoro_timer.toml");

        let config_pretty_string =
            toml::to_string_pretty(self).context("TOML serialization error")?;
        fs::write(&config_path, config_pretty_string)
            .with_context(|| format!("Failed to write to {}", config_path.display()))?;
        Ok(())
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

pub fn write_default() -> Result<()> {
    let config_and_status = ConfigAndStatus::default();
    config_and_status.write_to_disk()
}

pub fn increment_set() -> Result<()> {
    let mut state = get_state().unwrap_or_default();
    state.status.current_set += 1;
    state.write_to_disk()
}

pub fn reset_set() -> Result<()> {
    let mut state = get_state().unwrap_or_default();
    state.status.current_set = 1;
    state.write_to_disk()
}

pub fn write_config(
    sets: Option<u32>,
    work_time: Option<u64>,
    short_break_time: Option<u64>,
    long_break_time: Option<u64>,
) -> Result<()> {
    let mut state = get_state().unwrap_or_default();
    if let (None, None, None, None) = (sets, work_time, short_break_time, long_break_time) {
        println!("no changes to config");
        return Ok(());
    }
    // TODO: don't overwrite existing config
    let config = Config {
        sets,
        work: work_time,
        short_break: short_break_time,
        long_break: long_break_time,
    };
    state.config = Some(config);
    state.write_to_disk()?;
    print!("config set to:\n{}", toml::to_string_pretty(&state.config).unwrap());
    Ok(())
}
