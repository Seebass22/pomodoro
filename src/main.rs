extern crate clap;
use clap::{App, Arg, SubCommand};
use pomodoro::run;

fn main() {
    let matches = App::new("pomodoro")
        .arg(
            Arg::with_name("time")
                .short("t")
                .long("time")
                .value_name("TIME")
                .help("set the time"),
        )
        .subcommand(
            SubCommand::with_name("break")
                .about("take a 5 minute break"),
        )
        .subcommand(
            SubCommand::with_name("work")
                .about("work for 25 minutes"),
        )
        .get_matches();

    if matches.is_present("time") {
        if let Ok(t) = matches.value_of("time").unwrap().parse::<u64>() {
            run(t);
        } else {
            eprintln!("arg must be integer");
            return;
        }
    }

    if let Some(_matches) = matches.subcommand_matches("work") {
        run(25);
    }

    if let Some(_matches) = matches.subcommand_matches("break") {
        run(5);
    }
}
