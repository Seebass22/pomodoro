use notify_rust::Notification;
use std::{thread, time};

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

pub fn do_work(minutes: u64) {
    thread::sleep(time::Duration::from_secs(minutes * 60));

    let text = format!("{} minutes of work done", minutes);
    notify(&text);
}

pub fn take_break(minutes: u64) {
    thread::sleep(time::Duration::from_secs(minutes * 60));

    let text = format!("{} minute break over", minutes);
    notify(&text);
}
