use anyhow::Result;
use clap::{Parser, Subcommand};
use pomodoro::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

// update help text if changing defaults
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

    /// configure variables
    Config {
        #[arg(short, long)]
        sets: Option<u32>,

        #[arg(short, long = "work", value_name = "MINUTES")]
        work_time: Option<u64>,

        #[arg(short = 'b', long = "break", value_name = "MINUTES")]
        short_break_time: Option<u64>,

        #[arg(short, long = "long-break", value_name = "MINUTES")]
        long_break_time: Option<u64>,
    },
}

fn main() -> Result<()> {
    use Command::*;
    let config = match state::get_state() {
        Some(config) => config,
        None => {
            state::write_default()?;
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
                time,
                config.status.current_set,
                config.get_sets().unwrap_or(DEFAULT_SETS),
            )?;
        }

        Break { time, is_long } => {
            let time = if let Some(time) = time {
                time
            } else if is_long {
                config
                    .get_long_break_time()
                    .unwrap_or(DEFAULT_LONG_BREAK_TIME)
            } else {
                config
                    .get_short_break_time()
                    .unwrap_or(DEFAULT_SHORT_BREAK_TIME)
            };
            take_break(time, is_long)?;
        }

        Timer { time } => run(time)?,

        Config {
            sets,
            work_time,
            short_break_time,
            long_break_time,
        } => {
            state::write_config(sets, work_time, short_break_time, long_break_time)?;
        }
    }
    Ok(())
}
