use clap::{Parser, Subcommand};
use pomodoro::*;
mod state;

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
    },
    /// set a timer for any duration
    Timer {
        #[arg(short, long, value_name = "MINUTES")]
        time: u64,
    },
}

fn main() {
    use Command::*;
    let status = state::get_status();
    dbg!(status);

    let cli = Cli::parse();
    match cli.command {
        Work { time } => do_work(time),
        Break { time } => take_break(time),
        Timer { time } => run(time),
    }
}
