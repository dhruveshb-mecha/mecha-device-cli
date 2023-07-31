use clap::Args;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum CpuCommands {
    #[command(name = "info", about = "Get CPU information")]
    Info,
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct CpuArgs {
    #[command(subcommand)]
    command: Option<CpuCommands>,
}

impl CpuCommands {
    pub fn execute(&self) {
        match self {
            CpuCommands::Info => {
                println!("Getting CPU information...");
            }
        }
    }
}

impl CpuArgs {
    pub fn execute(&self) {
        if let Some(command) = &self.command {
            command.execute();
        } else {
            println!("Available commands:");
            println!("  info");
        }
    }
}