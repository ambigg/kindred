
use kindred::cmd_clap::Cli;
use clap::Parser;

fn main() {

    let cli = Cli::parse();
    println!("compiling... :p");

    if let Err(e) = kindred::execute(cli){
        eprintln!("compiler error :( \n{}",e );
        std::process::exit(1);
    }
}
