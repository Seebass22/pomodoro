extern crate clap;
use clap::{App, AppSettings, Arg, SubCommand};
use pomodoro::*;

fn main() {
    let matches = App::new("pomodoro")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("timer")
                .about("set a timer for any duration")
                .arg(
                    Arg::with_name("time")
                        .value_name("MINUTES")
                        .help("timer duration")
                        .required(true),
                ),
        )
        .subcommand(SubCommand::with_name("break").about("take a 5 minute break"))
        .subcommand(SubCommand::with_name("work").about("work for 25 minutes"))
        .get_matches();

    match matches.subcommand() {
        ("work", Some(_)) => do_work(),
        ("break", Some(_)) => take_break(),
        ("timer", Some(matches)) => {
            if let Ok(t) = matches.value_of("time").unwrap().parse::<u64>() {
                run(t);
            } else {
                eprintln!("arg must be integer");
                return;
            }
        }
        _ => (),
    }
}
