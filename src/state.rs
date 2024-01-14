use serde::{Deserialize, Serialize};
use std::error;

#[derive(Serialize, Deserialize, Default)]
struct ConfigAndStatus {
    config: Option<Config>,
    status: Status,
}

#[derive(Serialize, Deserialize)]
struct Config {
    set: Option<u32>,
    work: Option<u32>,
    short_break: Option<u32>,
    long_break: Option<u32>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Status {
    pub current_set: u32,
}

pub fn get_status() -> Option<Status> {
    fn get_status_res() -> Result<Status, Box<dyn error::Error>> {
        let mut toml_file = std::env::temp_dir();
        toml_file.push("pomodoro_timer.toml");
        let contents = std::fs::read_to_string(toml_file)?;
        let res = toml::from_str::<ConfigAndStatus>(&contents)?;
        Ok(res.status)
    }
    get_status_res().ok()
}

