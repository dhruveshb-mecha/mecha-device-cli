use clap::Parser;

// Include the `display.rs` module
mod display;
use display::DisplayArgs;   

mod battery;
use battery::BatteryArgs;

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
    #[command(name = "display", about = "display commands")]
    Display(DisplayArgs),
    #[command(name = "battery", about = "battery commands")]
    Battery(BatteryArgs), // Add the battery subcommand
}   

fn main() {
    let args = Cli::parse();
    println!("{:?}", args);

    match args.command {
        Commands::Display(display_args) => display_args.execute(),
        Commands::Battery(battery_args) => battery_args.execute(),
    }
}
