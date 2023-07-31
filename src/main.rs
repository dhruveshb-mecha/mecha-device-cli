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

mod device_info;
use device_info::DeviceInfoArgs;

mod audio;
use audio::AudioArgs;

mod cpu;
use cpu::CpuArgs;

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
    #[command(name = "device", about = "device info commands")]
    DeviceInfo(DeviceInfoArgs),
    #[command(name = "audio", about = "audio commands")]
    Audio(AudioArgs), // Add the audio subcommand
    #[command(name = "cpu", about = "cpu commands")]
    Cpu(CpuArgs), // Add the cpu subcommand

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
        Commands::DeviceInfo(device_info_args) => device_info_args.execute(),
        Commands::Audio(audio_args) => audio_args.execute(),
        Commands::Cpu(cpu_args) => cpu_args.execute(),
    }
}
