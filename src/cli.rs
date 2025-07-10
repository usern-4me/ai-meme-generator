use clap::{Parser, Subcommand};
use log::{info};


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(name= "MemeGenerator")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    ListTemplates,

    Generate{
        #[arg(short = 't', long, help= "image used as base")]
        template: String,

        #[arg(short = 'a', long, help= "text made by ai", default_value_t = false)]
        use_ai: bool,

        #[arg(short = 'c',long, help= "manual text")]
        text: Option<String>,
    },
}



pub fn run_cli() {
    let  cli = Cli::parse();
    info!("{:?}", cli);

    match cli.command {
        Commands::Generate { template, use_ai, text } => {
            info!("Generate command received!");
            info!("Template: {}", template);
            info!("Use AI?: {}", use_ai);

            match text {
                Some(text) => info!("Manual caption: {}", text),
                None => info!("No manual caption provided."),
            }

        }

        Commands::ListTemplates => {
            info!("Listing all available meme templates...");
        }
    }
}



