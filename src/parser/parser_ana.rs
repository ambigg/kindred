use crate::lexer::lexer_ana::{Lexer, TokenType};
use std::error::Error;

pub fn parser() -> Result<(), Box<dyn Error>> {
    let mut lexer = Lexer::from_file("main.kin")?;

    loop {
        let token = lexer.peek();

        match token.type_ {
            TokenType::EndOfFile => break,
            TokenType::Unknown => {
                // recoverable error  
                lexer.next_token();
            }
            TokenType::KeywordLet => {
                lexer.next_token();

                if lexer.peek().type_ != TokenType::Identifier {
                    eprintln!("error: identifier expected after 'let'");
                }

                lexer.next_token();
            }
            _ => {
                println!("Token: {:?} '{}'", token.type_, token.lexeme);
                lexer.next_token();
            }
        }
    }

    if lexer.has_errors() {
        eprintln!(
            "\nlexical error {} ",
            lexer.get_errors().len()
        );
        lexer.print_errors();
        return Err("lexical errors found".into());
    }

    Ok(())
}
