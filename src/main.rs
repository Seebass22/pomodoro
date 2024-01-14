use clap::{Parser, Subcommand};
use pomodoro::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// work for 25 minutes
    Work {
        #[arg(short, long, value_name = "MINUTES", default_value_t = 25)]
        time: u64,
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
            do_work(
                config.get_work_time().unwrap_or(time),
                config.status.current_set,
                config.get_sets().unwrap_or(4),
            );
        }
        Break { time, is_long } => {
            let default_time = if is_long { 20 } else { 5 };
            let time = if let Some(time) = time {
                time
            } else if is_long {
                config.get_long_break_time().unwrap_or(default_time)
            } else {
                config.get_short_break_time().unwrap_or(default_time)
            };
            take_break(time, is_long)
        }
        Timer { time } => run(time),
    }
}
