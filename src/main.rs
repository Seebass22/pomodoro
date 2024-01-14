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
        #[arg(short, long, value_name = "MINUTES", default_value_t = 5)]
        time: u64,

        #[arg(short, long, default_value_t = false)]
        long: bool,
    },
    /// set a timer for any duration
    Timer {
        #[arg(short, long, value_name = "MINUTES")]
        time: u64,
    },
}

fn main() {
    use Command::*;
    let state = match state::get_state() {
        Some(state) => state,
        None => {
            state::write_default().expect("IO error");
            state::ConfigAndStatus::default()
        }
    };

    let sets = if let Some(config) = state.config {
        config.sets.unwrap_or(4)
    } else {
        4
    };

    let cli = Cli::parse();
    match cli.command {
        Work { time } => {
            do_work(time, state.status.current_set, sets);
        }
        Break { time, long } => {
            take_break(time, long)
        },
        Timer { time } => run(time),
    }
}
