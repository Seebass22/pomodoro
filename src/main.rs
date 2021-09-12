extern crate clap;
use std::{thread, time};
use clap::{Arg, App};
use notify_rust::Notification;

fn main() {
    let matches = App::new("pomodoro")
        .arg(Arg::with_name("time")
             .short("t")
             .long("time")
             .value_name("TIME")
             .help("set the time time"))
        .get_matches();

    let duration;

    if let Ok(t) = 
        matches.value_of("time")
        .unwrap_or("2")
        .parse::<u64>() {
            duration = time::Duration::from_secs(t);
        } else {
            eprintln!("arg must be integer");
            return;
        }

    thread::sleep(duration);
    println!("timer ended");

    Notification::new()
        .summary("pomodoro")
        .body("pomodoro timer ended")
        .show()
        .unwrap();
}
