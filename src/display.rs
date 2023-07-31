// File: display.rs



use clap::Args;
use clap::Subcommand;
use mecha_display::Display;
use mecha_display::DisplayInterface;

#[derive(Debug, Subcommand)]
pub enum DisplayCommands {
    #[command(name = "on", about = "Turn on the display")]
    On,
    #[command(name = "off", about = "Turn off the display")]
    Off,
    #[command(name = "brightness", about = "Adjust the display brightness")]
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
    #[command(name = "get", about = "Get the current brightness level")]
    Get { brightness: Option<String> },
    #[command(name = "set", about = "Set the brightness level")]
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
            DisplayCommands::On => {
                println!("Display Off");
                let mut display = Display {
                    path: String::new(),
                };

                display.set_device("/sys/class/backlight/backlight/brightness");
                let _ = display.set_brightness(255);  

           
        },
            DisplayCommands::Off =>{
                println!("Display Off");
                let mut display = Display {
                    path: String::new(),
                };

                display.set_device("/sys/class/backlight/backlight/brightness");
                let _ = display.set_brightness(0);  


            },
            DisplayCommands::Brightness(brightness_args) => brightness_args.execute(),
        }
    }
}

impl BrightnessCommands {
    pub fn execute(&self) {
        match self {
            BrightnessCommands::Get { brightness: _ } => {
                let mut display = Display {
                    path: String::new(),
                };

                display.set_device("/sys/class/backlight/backlight/brightness");

                let current_brightness = display.get_brightness();
                println!("Brightness {:?}", current_brightness);
            },
            BrightnessCommands::Set { brightness } => {
                let mut display = Display {
                    path: String::new(),
                };

                display.set_device("/sys/class/backlight/backlight/brightness");

                let _ = display.set_brightness(brightness.clone().unwrap().parse::<u8>().unwrap());
            }
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
