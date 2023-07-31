use clap::Args;
use clap::Subcommand;


#[derive(Debug, Subcommand)]
pub enum AudioCommands {
    #[command(name = "play", about = "Play an audio file")]
    Play { file_path: String },
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct AudioArgs {
    #[command(subcommand)]
    command: Option<AudioCommands>,
}

impl AudioCommands {
    pub fn execute(&self) {
        match self {
            AudioCommands::Play { file_path } => {
                println!("Playing audio...");
                println!("File path: {}", file_path);
            }
        }
    }
}

impl AudioArgs {
    pub fn execute(&self) {
        if let Some(command) = &self.command {
            command.execute();
        } else {
            println!("Available commands:");
            println!("  play <file-path>");
        }
    }
}