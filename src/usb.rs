// // File: usb.rs

// use clap::Args;
// use clap::Subcommand;

// #[derive(Debug, Subcommand)]
// pub enum UsbCommands {
//     #[command(name = "host", about = "Set USB to host mode")]
//     Host(HostArgs),
//     #[command(name = "gadget", about = "Set USB to gadget mode")]
//     Gadget(GadgetArgs),
// }

// #[derive(Debug, Args)]
// #[command(args_conflicts_with_subcommands = true)]
// pub struct UsbArgs {
//     #[command(subcommand)]
//     command: Option<UsbCommands>,
// }

// #[derive(Debug, Subcommand)]
// pub enum HostCommands {
//     #[command(name = "set", about = "Set the host mode to the provided value")]
//     Set { value: u32 },
// }

// #[derive(Debug, Args)]
// #[command(args_conflicts_with_subcommands = true)]
// pub struct HostArgs {
//     #[command(subcommand)]
//     command: Option<HostCommands>,
// }

// #[derive(Debug, Subcommand)]
// pub enum GadgetCommands {
//     #[command(name = "set", about = "Set the gadget mode to the provided value")]
//     Set { value: u32 },
// }

// #[derive(Debug, Args)]
// #[command(args_conflicts_with_subcommands = true)]
// pub struct GadgetArgs {
//     #[command(subcommand)]
//     command: Option<GadgetCommands>,
// }

// impl UsbCommands {
//     pub fn execute(&self) {
//         match self {
//             UsbCommands::Host(host_args) => host_args.execute(),
//             UsbCommands::Gadget(gadget_args) => gadget_args.execute(),
//         }
//     }
// }

// impl HostCommands {
//     pub fn execute(&self) {
//         match self {
//             HostCommands::Set { value } => println!("Host mode set to: {}", value),
//         }
//     }
// }

// impl GadgetCommands {
//     pub fn execute(&self) {
//         match self {
//             GadgetCommands::Set { value } => println!("Gadget mode set to: {}", value),
//         }
//     }
// }

// impl HostArgs {
//     pub fn execute(&self) {
//         if let Some(command) = &self.command {
//             command.execute();
//         } else {
//             println!("Host");
//         }
//     }
// }

// impl GadgetArgs {
//     pub fn execute(&self) {
//         if let Some(command) = &self.command {
//             command.execute();
//         } else {
//             println!("Gadget");
//         }
//     }
// }

// impl UsbArgs {
//     pub fn execute(&self) {
//         if let Some(command) = &self.command {
//             command.execute();
//         } else {
//             println!("Usb");
//         }
//     }
// }

use clap::Args;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum UsbCommands {
    #[command(name = "host", about = "Set USB to host mode")]
    Host(HostArgs),
    #[command(name = "gadget", about = "Set USB to gadget mode")]
    Gadget(GadgetArgs),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct UsbArgs {
    #[command(subcommand)]
    command: Option<UsbCommands>,
}

#[derive(Debug, Subcommand)]
pub enum HostCommands {
    #[command(name = "set", about = "Set the host mode to the provided value")]
    Set { value: u32 },
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct HostArgs {
    #[command(subcommand)]
    command: Option<HostCommands>,
}

#[derive(Debug, Subcommand)]
pub enum GadgetCommands {
    #[command(name = "set", about = "Set the gadget mode to the provided value")]
    Set { value: u32 },
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct GadgetArgs {
    #[command(subcommand)]
    command: Option<GadgetCommands>,
}

impl UsbCommands {
    pub fn execute(&self) {
        match self {
            UsbCommands::Host(host_args) => host_args.execute(),
            UsbCommands::Gadget(gadget_args) => gadget_args.execute(),
        }
    }
}

impl HostCommands {
    pub fn execute(&self) {
        match self {
            HostCommands::Set { value } => println!("Host mode set to: {}", value),
        }
    }
}

impl GadgetCommands {
    pub fn execute(&self) {
        match self {
            GadgetCommands::Set { value } => println!("Gadget mode set to: {}", value),
        }
    }
}

impl HostArgs {
    pub fn execute(&self) {
        if let Some(command) = &self.command {
            command.execute();
        } else {
            println!("Host");
        }
    }
}

impl GadgetArgs {
    pub fn execute(&self) {
        if let Some(command) = &self.command {
            command.execute();
        } else {
            println!("Gadget");
        }
    }
}

impl UsbArgs {
    pub fn execute(&self) {
        if let Some(command) = &self.command {
            command.execute();
        } else {
            println!("Available commands:");
            println!("  host");
            println!("  gadget");
        }
    }
}

