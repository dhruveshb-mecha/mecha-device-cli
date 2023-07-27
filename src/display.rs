// File: display.rs

use clap::Args;

use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum DisplayCommands {
    On,
    Off,
    Brightness(BrightnessArgs),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct DisplayArgs {
    #[command(subcommand)]
    pub command: Option<DisplayCommands>,
}

#[derive(Debug, Subcommand)]
pub enum BrightnessCommands {
    Get { brightness: Option<String> },
    Set { brightness: Option<String> },
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct BrightnessArgs {
    #[command(subcommand)]
    pub command: Option<BrightnessCommands>,
}
