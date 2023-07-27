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
    command: Option<DisplayCommands>,
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
    command: Option<BrightnessCommands>,
}

impl DisplayCommands {
    fn execute(&self) {
        match self {
            DisplayCommands::On => println!("Display On"),
            DisplayCommands::Off => println!("Display Off"),
            DisplayCommands::Brightness(brightness_args) => brightness_args.execute(),
        }
    }
}

impl BrightnessCommands {
    pub  fn execute(&self) {
        match self {
            BrightnessCommands::Get { brightness } => println!("Brightness Get {:?}", brightness),
            BrightnessCommands::Set { brightness } => println!("Brightness Set {:?}", brightness),
        }
    }
}

impl BrightnessArgs {
    fn execute(&self) {
        if let Some(command) = &self.command {
            command.execute();
        } else {
            println!("Brightness");
        }
    }
}

impl DisplayArgs {
    pub fn execute(&self) {
        if let Some(command) = &self.command {
            command.execute();
        } else {
            println!("Display");
        }
    }
}
