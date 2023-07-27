// File: battery.rs

use clap::Args;
use clap::Parser;
use clap::Subcommand;
use clap::ValueEnum;

#[derive(Debug, Subcommand)]
pub enum BatteryCommands {
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
