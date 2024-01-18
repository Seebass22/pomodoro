use anyhow::Result;
use notify_rust::Notification;
use std::{thread, time};
pub mod state;

fn notify(text: &str) {
    Notification::new()
        .summary("pomodoro")
        .body(text)
        .show()
        .unwrap();
}

pub fn run(minutes: u64) {
    thread::sleep(time::Duration::from_secs(minutes * 60));

    let text = format!("{} minute timer ended", minutes);
    notify(&text);
}

pub fn do_work(minutes: u64, current_set: u32, sets: u32) -> Result<()> {
    println!("working for {} minutes [{}/{}]", minutes, current_set, sets);
    thread::sleep(time::Duration::from_secs(minutes * 60));

    let text = if current_set >= sets {
        format!("{} minutes of work done\nset finished: next break should be long (pomodoro break --long)", minutes)
    } else {
        format!("{} minutes of work done", minutes)
    };

    state::increment_set()?;

    println!("{}", &text);
    notify(&text);
    Ok(())
}

pub fn take_break(minutes: u64, is_long: bool) -> Result<()> {
    if is_long {
        println!("taking a long break [{} minutes]", minutes);
    } else {
        println!("taking a break [{} minutes]", minutes);
    }

    thread::sleep(time::Duration::from_secs(minutes * 60));

    if is_long {
        state::reset_set()?;
    }
    let text = format!("{} minute break over", minutes);
    println!("{}", &text);
    notify(&text);
    Ok(())
}
