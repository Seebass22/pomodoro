extern crate clap;
use std::{thread, time};
use clap::{Arg, App};

fn main() {
    let matches = App::new("pomodoro")
        .arg(Arg::with_name("time")
             .short("t")
             .long("time")
             .value_name("TIME")
             .help("set the time time"))
        .get_matches();

    let time : u64;

    if let Ok(t) = 
        matches.value_of("time")
        .unwrap_or("2")
        .parse::<u64>() {
            time = t;
        } else {
            eprintln!("arg must be integer");
            return;
        }

    let duration = time::Duration::from_secs(time);

    thread::sleep(duration);
    println!("timer ended");
}
