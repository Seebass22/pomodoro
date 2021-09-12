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
        .get_matches();

    if let Ok(t) = matches.value_of("time").unwrap_or("25").parse::<u64>() {
        run(t);
    } else {
        eprintln!("arg must be integer");
        return;
    }
}
