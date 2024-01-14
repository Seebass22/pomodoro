use clap::{Parser, Subcommand};
use pomodoro::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

const DEFAULT_SETS: u32 = 4;
const DEFAULT_WORK_TIME: u64 = 25;
const DEFAULT_SHORT_BREAK_TIME: u64 = 5;
const DEFAULT_LONG_BREAK_TIME: u64 = 20;

#[derive(Subcommand)]
enum Command {
    /// work for 25 minutes
    Work {
        /// [default: 25]
        #[arg(short, long, value_name = "MINUTES")]
        time: Option<u64>,
    },
    /// take a 5 minute break
    Break {
        /// [default: 5 (short break) or 20 (long break)]
        #[arg(short, long, value_name = "MINUTES")]
        time: Option<u64>,

        /// take a long break
        #[arg(short = 'l', long = "long", default_value_t = false)]
        is_long: bool,
    },
    /// set a timer for any duration
    Timer {
        #[arg(short, long, value_name = "MINUTES")]
        time: u64,
    },
}

fn main() {
    use Command::*;
    let config = match state::get_state() {
        Some(config) => config,
        None => {
            state::write_default().expect("IO error");
            state::ConfigAndStatus::default()
        }
    };

    let cli = Cli::parse();
    match cli.command {
        Work { time } => {
            let time = if let Some(time) = time {
                time
            } else {
                config.get_work_time().unwrap_or(DEFAULT_WORK_TIME)
            };
            do_work(
                config.get_work_time().unwrap_or(time),
                config.status.current_set,
                config.get_sets().unwrap_or(DEFAULT_SETS),
            );
        }

        Break { time, is_long } => {
            let time = if let Some(time) = time {
                time
            } else if is_long {
                config.get_long_break_time().unwrap_or(DEFAULT_LONG_BREAK_TIME)
            } else {
                config.get_short_break_time().unwrap_or(DEFAULT_SHORT_BREAK_TIME)
            };
            take_break(time, is_long)
        }

        Timer { time } => run(time),
    }
}
