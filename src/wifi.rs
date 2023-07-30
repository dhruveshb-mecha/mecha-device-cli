use clap::Args;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum WifiCommands {
    #[command(name = "scan", about = "Scan for WiFi networks")]
    Scan,
    #[command(name = "connect", about = "Connect to a WiFi network")]
    Connect { device_id: String },
    #[command(name = "disconnect", about = "Disconnect from a WiFi network")]
    Disconnect { device_id: String },
    #[command(name = "remove", about = "Remove a WiFi network")]
    Remove { device_id: String },
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct WifiArgs {
    #[command(subcommand)]
    command: Option<WifiCommands>,
}

impl WifiCommands {
    pub fn execute(&self) {
        match self {
            WifiCommands::Scan => println!("Scanning for WiFi networks..."),
            WifiCommands::Connect { device_id } => println!("Connecting to WiFi network: {}", device_id),
            WifiCommands::Disconnect { device_id } => println!("Disconnecting from WiFi network: {}", device_id),
            WifiCommands::Remove { device_id } => println!("Removing WiFi network: {}", device_id),
        }
    }
}

impl WifiArgs {
    pub fn execute(&self) {
        if let Some(command) = &self.command {
            command.execute();
        } else {
            println!("Available commands:");
            println!("  scan");
            println!("  connect <device-id>");
            println!("  disconnect <device-id>");
            println!("  remove <device-id>");
        }
    }
}