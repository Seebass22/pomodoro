extern crate clap;
use clap::{App, Arg};
use pomodoro::run;

fn main() {
    let matches = App::new("pomodoro")
        .arg(
            Arg::with_name("time")
                .short("t")
                .long("time")
                .value_name("TIME")
                .help("set the time time"),
        )
        .arg(
            Arg::with_name("break")
                .short("b")
                .long("break")
                .help("take a 5 minute break"),
        )
        .arg(
            Arg::with_name("work")
                .short("w")
                .long("work")
                .help("work for 25 minutes"),
        )
        .get_matches();

    if matches.is_present("break") {
        run(5);
    } else if matches.is_present("work") {
        run(25);
    } else if matches.is_present("time") {
        if let Ok(t) = matches.value_of("time").unwrap().parse::<u64>() {
            run(t);
        } else {
            eprintln!("arg must be integer");
            return;
        }
    } else {
        eprintln!("?");
        return;
    }
}
