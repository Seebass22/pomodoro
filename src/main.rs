use clap::{App, AppSettings, Arg, SubCommand};
use pomodoro::*;

fn is_int(val: String) -> Result<(), String> {
    if val.parse::<u64>().is_ok() {
        Ok(())
    } else {
        Err(String::from("arg must be integer"))
    }
}

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
                        .required(true)
                        .validator(is_int),
                ),
        )
        .subcommand(
            SubCommand::with_name("break")
                .about("take a 5 minute break")
                .arg(
                    Arg::with_name("time")
                        .short("t")
                        .long("time")
                        .value_name("MINUTES")
                        .help("set a different duration")
                        .default_value("5")
                        .validator(is_int),
                ),
        )
        .subcommand(
            SubCommand::with_name("work")
                .about("work for 25 minutes")
                .arg(
                    Arg::with_name("time")
                        .short("t")
                        .long("time")
                        .value_name("MINUTES")
                        .help("set a different duration")
                        .default_value("25")
                        .validator(is_int),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("work", Some(matches)) => {
            let t = matches.value_of("time").unwrap().parse::<u64>().unwrap();
            do_work(t);
        }
        ("break", Some(matches)) => {
            let t = matches.value_of("time").unwrap().parse::<u64>().unwrap();
            take_break(t);
        }
        ("timer", Some(matches)) => {
            let t = matches.value_of("time").unwrap().parse::<u64>().unwrap();
            run(t);
        }
        _ => (),
    }
}
