use clap::Args;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum BluetoothCommands {
    #[command(name = "scan", about = "Scan for Bluetooth devices")]
    Scan,
    #[command(name = "add", about = "Add a Bluetooth device")]
    Add { device_id: String },
    #[command(name = "remove", about = "Remove a Bluetooth device")]
    Remove { device_id: String },
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct BluetoothArgs {
    #[command(subcommand)]
    command: Option<BluetoothCommands>,
}

impl BluetoothCommands {
    pub fn execute(&self) {
        match self {
            BluetoothCommands::Scan => println!("Scanning for Bluetooth devices..."),
            BluetoothCommands::Add { device_id } => println!("Adding Bluetooth device: {}", device_id),
            BluetoothCommands::Remove { device_id } => println!("Removing Bluetooth device: {}", device_id),
        }
    }
}

impl BluetoothArgs {
    pub fn execute(&self) {
        if let Some(command) = &self.command {
            command.execute();
        } else {
            println!("Available commands:");
            println!("  scan");
            println!("  add <device-id>");
            println!("  remove <device-id>");
        }
    }
}