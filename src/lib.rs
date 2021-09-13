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
    let duration = time::Duration::from_secs(minutes * 60);
    thread::sleep(duration);
    println!("timer ended");

    let text = format!("{} minute pomodoro timer ended", minutes);
    notify(&text);
}
