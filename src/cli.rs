use clap::{Parser, Subcommand};
use log::{info};


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(name= "TextRenderer")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    ListImages,

    Generate{
        #[arg(short = 'I', long, help= "image used as base")]
        Image: String,

        #[arg(short = 't',long, help= "text to render")]
        text: String,
    },
}



pub fn run_cli() {
    let  cli = Cli::parse();
    info!("{:?}", cli);

    match cli.command {
        Commands::Generate { Image, text } => {
            info!("Image: {}", Image);
            info!("text: {}", text);

        }

        Commands::ListImages => {
            crate::commands::list::list_images_fn();
        }
    }
}



