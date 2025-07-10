mod cli;

use cli::Cli;
use cli::Commands;


use clap::Parser;

fn main() {
    let  cli = Cli::parse();
    println!("{:?}", cli);

    match cli.command {
        Commands::Generate { template, use_ai, text } => {
            println!("Generate command received!");
            println!("Template: {}", template);
            println!("Use AI?: {}", use_ai);

            match text {
                Some(text) => println!("Manual caption: {}", text),
                None => println!("No manual caption provided."),
            }

        }

        Commands::ListTemplates => {
            println!("Listing all available meme templates...");
        }
    }
}



