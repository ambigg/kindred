pub mod cmd_clap;
pub mod compiler;
pub mod lexer;
pub mod parser;
pub mod util;

use cmd_clap::{Cli, Commands};
use std::error::Error;
use std::fs;


//cmd to create the executable (run the compiler basically)
fn make_cmd(mode: &str) -> Result<(), Box<dyn Error>> {
    // let main_path = "src/main.kin";
    compiler::compile(mode);

    Ok(())
}

// fn run_cmd(target: Option<&str>) -> Result<(), Box<dyn Error>> {
//     let t = target.unwrap_or("default");
//     println!("Here will be the running of the executable");
//     Ok(())
// }

//cmd to clean the exutable
fn clean_cmd() -> Result<(), Box<dyn Error>> {
    let path = "make.ob";
    fs::remove_file(path)?;
    println!("Executable deleted");
    Ok(())
}

//cmd to run the project on the main.rs
pub fn execute(cli: Cli) -> Result<(), Box<dyn Error>> {
    println!("running");
    match cli.command {
        Commands::Make { mode } => make_cmd(&mode),
        // Commands::Run { target } => run_cmd(target.as_deref()),
        Commands::Clean => clean_cmd(),
    }
}
