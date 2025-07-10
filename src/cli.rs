use clap::{Parser, Subcommand};

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







