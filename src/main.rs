// use std::ffi::OsStr;
// use std::ffi::OsString;
// use std::path::PathBuf;

// use clap::{Args, Parser, Subcommand, ValueEnum};

// /// A fictional versioning CLI
// #[derive(Debug, Parser)] // requires `derive` feature
// #[command(name = "mecha")]
// #[command(about = "A fictional versioning CLI", long_about = None)]
// struct Cli {
//     #[command(subcommand)]
//     command: Commands,
// }

// #[derive(Debug, Subcommand)]
// enum Commands {
//     Display(DisplayArgs),
// }

// #[derive(Debug, Args)]
// #[command(args_conflicts_with_subcommands = true)]
// struct DisplayArgs {
//     #[command(subcommand)]
//     command: Option<DisplayCommands>,
// }

// #[derive(Debug, Subcommand)]
// enum DisplayCommands {
//     /// turn on display
//     On,
//     /// turn off display
//     Off,
// /// Brightness this has two sub commands set and get
//     Brightness(BrightnessArgs),
// }

// #[derive(Debug, Args)]
// #[command(args_conflicts_with_subcommands = true)]
// struct BrightnessArgs {
//     #[command(subcommand)]
//     command: Option<BrightnessCommands>,

//     // #[command(flatten)]
//     // get: SetBrightnessArgs,
// }

// #[derive(Debug, Subcommand)]
// enum BrightnessCommands {
//     /// get current brightness
//     Get  { brightness: Option<String> },
//     /// set brightness to a value
//     Set { brightness: Option<String> },

// }

// #[derive(Debug, Args)]
// struct SetBrightnessArgs {
//     #[arg(short, long)]
//     brightness: Option<String>,
// }

// fn main() {
//     let args = Cli::parse();
//     println!("{:?}", args);

//     match args.command {
//         Commands::Display(display_args) => match display_args.command {
//             Some(display_command) => match display_command {
//                 DisplayCommands::On => println!("Display On"),
//                 DisplayCommands::Off => println!("Display Off"),
//                 DisplayCommands::Brightness(brightness_args) => match brightness_args.command {
//                     Some(brightness_command) => match brightness_command {
//                         BrightnessCommands::Get { brightness } => println!("Brightness Get {:?}", brightness),
//                         BrightnessCommands::Set { brightness } => println!("Brightness Set {:?}", brightness),
//                     },
//                     None => println!("Brightness"),
//                 },
//             },
//             None => println!("Display"),
//         },
//     }
// }

use clap::Parser;

// Include the `display.rs` module
mod display;
use display::{ BrightnessCommands, DisplayArgs, DisplayCommands};   

mod battery;
use battery::{BatteryArgs, BatteryCommands};

/// A fictional versioning CLI
#[derive(Debug, Parser)]
#[command(name = "mecha")]
#[command(about = "A fictional versioning CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}       

#[derive(Debug, Parser)]
enum Commands {
    Display(DisplayArgs),
    Battery(BatteryArgs), // Add the battery subcommand
}   

fn main() {
    let args = Cli::parse();
    println!("{:?}", args);

    match args.command {
        Commands::Display(display_args) => match display_args.command {
            Some(display_command) => match display_command {
                DisplayCommands::On => println!("Display On"),
                DisplayCommands::Off => println!("Display Off"),
                DisplayCommands::Brightness(brightness_args) => match brightness_args.command {
                    Some(brightness_command) => match brightness_command {
                        BrightnessCommands::Get { brightness } => {
                            println!("Brightness Get {:?}", brightness)
                        }
                        BrightnessCommands::Set { brightness } => {
                            println!("Brightness Set {:?}", brightness)
                        }
                    },
                    None => println!("Brightness"),
                },
            },
            None => println!("Display"),
        },
        Commands::Battery(battery_args) => match battery_args.command {
            Some(battery_command) => match battery_command {
                BatteryCommands::Info => println!("Battery Info"),
            },
            None => println!("Battery"),
        },
    }
}
