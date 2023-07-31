use clap::Args;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum DeviceInfoCommands {
    #[command(name = "info", about = "Get device information")]
    Info,
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct DeviceInfoArgs {
    #[command(subcommand)]
    command: Option<DeviceInfoCommands>,
}

impl DeviceInfoCommands {
    pub fn execute(&self) {
        match self {
            DeviceInfoCommands::Info => println!("Getting device information..."),
        }
    }
}

impl DeviceInfoArgs {
    pub fn execute(&self) {
        if let Some(command) = &self.command {
            command.execute();
        } else {
            println!("Available commands:");
            println!("  info");
        }
    }
}