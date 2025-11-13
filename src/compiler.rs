// use crate::lexer::lexer_ana;
use crate::parser::parser_ana;
// use crate::util::symboltable::SymbolTable;

// use std::error::Error;   

pub fn compile(mode: &str) {
    println!("compiling in mode: {}", mode);
    
    match parser_ana::parser() {
        Ok(_) => println!("successful compilation "),
        Err(e) => eprintln!(" error: {}", e),
    }
}

