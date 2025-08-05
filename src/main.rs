mod kill;
mod pause;
mod show;
use clap::{Parser, Subcommand};
use kill::{kill_process_by_name, kill_process_by_pid};
use pause::{
    pause_process_by_name, pause_process_by_pid, resume_process_by_name, resume_process_by_pid,
};
use show::show_processes;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Show all the processes of current oprating system")]
    Proc,
    #[command(about = "Kill a process by its PID")]
    Kbp { pid: u32 },
    #[command(about = "Kill a process by its name")]
    Kbn { name: String },
    #[command(about = "Pause a process by its PID")]
    Pbp { pid: u32 },
    #[command(about = "Pause a process by its name")]
    Pbn { name: String },
    #[command(about = "Resume a process by its PID")]
    Rbp { pid: u32 },
    #[command(about = "Resume a process by its name")]
    Rbn { name: String },
}

fn main() -> () {
    let args = Args::parse();
    match args.command {
        Commands::Proc => {
            show_processes();
        }
        Commands::Kbp { pid } => {
            kill_process_by_pid(pid);
        }
        Commands::Kbn { name } => {
            kill_process_by_name(name);
        }
        Commands::Pbp { pid } => {
            pause_process_by_pid(pid);
        }
        Commands::Pbn { name } => {
            pause_process_by_name(name);
        }
        Commands::Rbp { pid } => {
            resume_process_by_pid(pid);
        }
        Commands::Rbn { name } => {
            resume_process_by_name(name);
        }
    }
}
