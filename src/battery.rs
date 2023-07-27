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
    pub command: Option<BatteryCommands>,
}
