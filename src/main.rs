use clap::Parser;

// Include the `display.rs` module
mod display;
use display::DisplayArgs;   

mod battery;
use battery::BatteryArgs;

mod usb;
use usb::UsbArgs;

mod bluetooth;
use bluetooth::BluetoothArgs;

mod wifi;
use wifi::WifiArgs;

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
    #[command(name = "usb", about = "usb commands")]
    Usb(UsbArgs), // Add the usb subcommand
    #[command(name = "bt", about = "bluetooth commands")]
    Bluetooth(BluetoothArgs), // Add the bluetooth subcommand
    #[command(name = "wifi", about = "wifi commands")]
    Wifi(WifiArgs), // Add the wifi subcommand

}   

fn main() {
    let args = Cli::parse();
    println!("{:?}", args);

    match args.command {
        Commands::Display(display_args) => display_args.execute(),
        Commands::Battery(battery_args) => battery_args.execute(),
        Commands::Usb(usb_args) => usb_args.execute(),
        Commands::Bluetooth(bluetooth_args) => bluetooth_args.execute(),
        Commands::Wifi(wifi_args) => wifi_args.execute(),
    }
}
