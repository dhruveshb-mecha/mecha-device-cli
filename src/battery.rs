// File: battery.rs

use clap::Args;
use clap::Subcommand;


#[derive(Debug, Subcommand)]
pub enum BatteryCommands {
    #[command(name = "info", about = "Battery Info")]
    Info,
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct BatteryArgs {
    #[command(subcommand)]
    command: Option<BatteryCommands>,
}

impl BatteryCommands {
    fn execute(&self) {
        match self {
            BatteryCommands::Info => println!("Battery Info"),
        }
    }
}

impl BatteryArgs {
   pub fn execute(&self) {
        if let Some(command) = &self.command {
            command.execute();
        } else {
            println!("Battery");
        }
    }
}
