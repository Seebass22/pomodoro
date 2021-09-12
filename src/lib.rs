use notify_rust::Notification;
use std::{thread, time};

pub fn run(minutes: u64) {
    let duration = time::Duration::from_secs(minutes * 60);
    thread::sleep(duration);
    println!("timer ended");

    Notification::new()
        .summary("pomodoro")
        .body("pomodoro timer ended")
        .show()
        .unwrap();
}
