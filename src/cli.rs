use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Ai Meme Generator")]
#[command(about = "generates memes with captions using Ai", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand)]
pub enum Commands {

    ListTemplates,


    Generate{
        #[arg(short, long)]
        template: String,


        #[arg(short, help = "Use AI to generate caption")]
        use_ai: bool,

        #[arg(short, long, help = "Manual Caption for meme")]
        text: Option<String>,

    },
}