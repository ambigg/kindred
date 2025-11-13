use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author,version,about = "Kindred commands", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Make {
        #[arg(short, long, default_value = "Release")]
        mode: String,
    },
    // Run {
    //     #[arg(short, long)]
    //     target: Option<String>,
    // },

    Clean,
}
