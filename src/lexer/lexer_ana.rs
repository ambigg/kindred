use std::{collections::HashMap, error::Error, fmt, fs};
// use crate::lexer::helper::lexer_helper;
use crate::lexer::automaton::{Automata, DfaRunner, TransitionResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

impl Span {
    pub fn new(start: usize, end: usize, line: usize, column: usize) -> Self {
        Span {
            start,
            end,
            line,
            column,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorType {
    UnexpectedCharacter,
    UnterminatedString,
    InvalidNumber,
    InvalidEscape,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct LexerError {
    pub message: String,
    pub span: Span,
    pub error_type: ErrorType,
}

impl LexerError {
    pub fn new(message: String, span: Span, error_type: ErrorType) -> Self {
        LexerError {
            message,
            span,
            error_type,
        }
    }

    pub fn display(&self, source: &str) {
        eprintln!(
            "\n Lexical error on line {}:{}",
            self.span.line, self.span.column
        );
        eprintln!("     {}", self.message);

        if let Some(line_text) = source.lines().nth(self.span.line - 1) {
            eprintln!("\n   {}", line_text);
            eprintln!("   {}^", " ".repeat(self.span.column - 1));
        }
    }
}

// const FILE_PATH: &str = "main.kin";

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    KeywordLet,
    KeywordIf,
    KeywordElse,
    KeywordFor,
    KeywordWhile,
    KeywordReturn,
    KeywordFn,

    Identifier,
    Integer,
    Float,
    StringLiteral,

    OperatorPlus,      // +
    OperatorMinus,     // -
    OperatorMultiply,  // *
    OperatorDivide,    // /
    OperatorAssign,    // =
    OperatorEqual,     // ==
    OperatorNotEqual,  // !=
    OperatorLess,      // <
    OperatorGreater,   // >
    OperatorLessEq,    // <=
    OperatorGreaterEq, // >=

    ParenthesisLeft,  // (
    ParenthesisRight, // )
    BraceLeft,        // {
    BraceRight,       // }
    BracketLeft,
    BracketRight,
    Semicolon, // ;
    Comma,     // ,
    Dot,
    Or,
    Not,
    And,

    EndOfFile,
    Unknown,

    // School-specific keywords
    Programa_,
    Define_,
    Maquinas_,
    Concentradores_,
    Coaxial_,
    Modulo_,
    Inicio_,
    Fin_,
    Coloca_,
    ColocaCoaxial_,
    ColocaCoaxialConcentrador_,
    UneMaquinaPuerto_,
    AsignaPuerto_,
    MaquinaCoaxial_,
    AsignaMaquinaCoaxial_,
    Escribe_,
    Si_,
    Sino_,
    Arriba_,
    Abajo_,
    Izquierda_,
    Derecha_,
    Puertos_,
    Disponibles_,
    Presente_,
    Longitud_,
    Completo_,
    Num_,
    Maquina_,
    Pos_,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub type_: TokenType,
    pub lexeme: String,
    pub span: Span,
}

impl Token {
    pub fn new(type_: TokenType, lexeme: String, span: Span) -> Self {
        Token {
            type_,
            lexeme,
            span,
        }
    }

    pub fn eof(position: usize, line: usize, column: usize) -> Self {
        Token {
            type_: TokenType::EndOfFile,
            lexeme: String::new(),
            span: Span::new(position, position, line, column),
        }
    }

    pub fn error(lexeme: String, span: Span) -> Self {
        Token {
            type_: TokenType::Unknown,
            lexeme,
            span,
        }
    }
}

pub struct Lexer {
    source: Vec<char>,
    source_string: String,
    current_index: usize,
    line: usize,
    column: usize,

    peeked_token: Option<Token>,

    identifier_dfa: Automata,
    integer_dfa: Automata,
    float_dfa: Automata,

    keywords: HashMap<String, TokenType>,
    errors: Vec<LexerError>,
}

impl Lexer {
    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let source_code = fs::read_to_string(file_path)?;
        Self::new(&source_code)
    }

    pub fn new(source_code: &str) -> Result<Self, Box<dyn Error>> {
        let identifier_dfa = Self::load_identifier_dfa()?;
        let integer_dfa = Self::load_integer_dfa()?;
        let float_dfa = Self::load_float_dfa()?;
        let mut keywords = HashMap::new();

        keywords.insert("let".to_string(), TokenType::KeywordLet);
        keywords.insert("if".to_string(), TokenType::KeywordIf);
        keywords.insert("else".to_string(), TokenType::KeywordElse);
        keywords.insert("for".to_string(), TokenType::KeywordFor);
        keywords.insert("while".to_string(), TokenType::KeywordWhile);
        keywords.insert("return".to_string(), TokenType::KeywordReturn);
        keywords.insert("fn".to_string(), TokenType::KeywordFn);
        keywords.insert("define".to_string(), TokenType::Define_);
        keywords.insert("programa".to_string(), TokenType::Programa_);
        keywords.insert("inicio".to_string(), TokenType::Inicio_);
        keywords.insert("fin".to_string(), TokenType::Fin_);
        keywords.insert("modulo".to_string(), TokenType::Modulo_);
        keywords.insert("maquinas".to_string(), TokenType::Maquinas_);
        keywords.insert("concentradores".to_string(), TokenType::Concentradores_);
        keywords.insert("coaxial".to_string(), TokenType::Coaxial_);
        keywords.insert("coloca".to_string(), TokenType::Coloca_);
        keywords.insert("colocaCoaxial".to_string(), TokenType::ColocaCoaxial_);
        keywords.insert(
            "colocaCoaxialConcentrador".to_string(),
            TokenType::ColocaCoaxialConcentrador_,
        );
        keywords.insert("uneMaquinaPuerto".to_string(), TokenType::UneMaquinaPuerto_);
        keywords.insert("asignaPuerto".to_string(), TokenType::AsignaPuerto_);
        keywords.insert("maquinaCoaxial".to_string(), TokenType::MaquinaCoaxial_);
        keywords.insert(
            "asignaMaquinaCoaxial".to_string(),
            TokenType::AsignaMaquinaCoaxial_,
        );
        keywords.insert("escribe".to_string(), TokenType::Escribe_);
        keywords.insert("si".to_string(), TokenType::Si_);
        keywords.insert("sino".to_string(), TokenType::Sino_);
        keywords.insert("arriba".to_string(), TokenType::Arriba_);
        keywords.insert("abajo".to_string(), TokenType::Abajo_);
        keywords.insert("izquierda".to_string(), TokenType::Izquierda_);
        keywords.insert("derecha".to_string(), TokenType::Derecha_);
        keywords.insert("puertos".to_string(), TokenType::Puertos_);
        keywords.insert("disponibles".to_string(), TokenType::Disponibles_);
        keywords.insert("presente".to_string(), TokenType::Presente_);
        keywords.insert("longitud".to_string(), TokenType::Longitud_);
        keywords.insert("completo".to_string(), TokenType::Completo_);
        keywords.insert("num".to_string(), TokenType::Num_);
        keywords.insert("maquina".to_string(), TokenType::Maquina_);
        keywords.insert("pos".to_string(), TokenType::Pos_);

        Ok(Lexer {
            source: source_code.chars().collect(),
            source_string: source_code.to_string(),
            current_index: 0,
            line: 1,
            column: 1,
            peeked_token: None,
            identifier_dfa,
            integer_dfa,
            float_dfa,
            keywords,
            errors: Vec::new(),
        })
    }

    pub fn peek(&mut self) -> &Token {
        if self.peeked_token.is_none() {
            self.peeked_token = Some(self.scan_token());
        }
        self.peeked_token.as_ref().unwrap()
    }

    pub fn next_token(&mut self) -> Token {
        if let Some(token) = self.peeked_token.take() {
            return token;
        }
        self.scan_token()
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn get_errors(&self) -> &[LexerError] {
        &self.errors
    }

    pub fn print_errors(&self) {
        for error in &self.errors {
            error.display(&self.source_string);
        }
    }

    fn scan_token(&mut self) -> Token {
        self.skip_whitespace_and_comments();

        if self.is_at_end() {
            return Token::eof(self.current_index, self.line, self.column);
        }

        let start_index = self.current_index;
        let start_line = self.line;
        let start_column = self.column;

        match self.peek_char() {
            '(' => {
                self.advance();
                self.make_token(
                    TokenType::ParenthesisLeft,
                    "(",
                    start_index,
                    start_line,
                    start_column,
                )
            }
            ')' => {
                self.advance();
                self.make_token(
                    TokenType::ParenthesisRight,
                    ")",
                    start_index,
                    start_line,
                    start_column,
                )
            }
            '{' => {
                self.advance();
                self.make_token(
                    TokenType::BraceLeft,
                    "{",
                    start_index,
                    start_line,
                    start_column,
                )
            }
            '}' => {
                self.advance();
                self.make_token(
                    TokenType::BraceRight,
                    "}",
                    start_index,
                    start_line,
                    start_column,
                )
            }
            '[' => {
                self.advance();
                self.make_token(
                    TokenType::BracketLeft,
                    "[",
                    start_index,
                    start_line,
                    start_column,
                )
            }
            ']' => {
                self.advance();
                self.make_token(
                    TokenType::BracketRight,
                    "]",
                    start_index,
                    start_line,
                    start_column,
                )
            }
            '&' => {
                self.advance();
                if !self.is_at_end() && self.peek_char() == '&' {
                    self.advance();
                    self.make_token(TokenType::And, "&&", start_index, start_line, start_column)
                } else {
                    self.report_error(
                        "Expected '&&'".to_string(),
                        start_index,
                        start_line,
                        start_column,
                        ErrorType::UnexpectedCharacter,
                    )
                }
            }
            ';' => {
                self.advance();
                self.make_token(
                    TokenType::Semicolon,
                    ";",
                    start_index,
                    start_line,
                    start_column,
                )
            }
            ',' => {
                self.advance();
                self.make_token(TokenType::Comma, ",", start_index, start_line, start_column)
            }
            '.' => {
                self.advance();
                self.make_token(TokenType::Dot, ".", start_index, start_line, start_column)
            }
            '|' => {
                self.advance();
                if !self.is_at_end() && self.peek_char() == '|' {
                    self.advance();
                    self.make_token(TokenType::Or, "||", start_index, start_line, start_column)
                } else {
                    self.report_error(
                        "Expected '||'".to_string(),
                        start_index,
                        start_line,
                        start_column,
                        ErrorType::UnexpectedCharacter,
                    )
                }
            }
            '+' => {
                self.advance();
                self.make_token(
                    TokenType::OperatorPlus,
                    "+",
                    start_index,
                    start_line,
                    start_column,
                )
            }
            '-' => {
                self.advance();
                self.make_token(
                    TokenType::OperatorMinus,
                    "-",
                    start_index,
                    start_line,
                    start_column,
                )
            }
            '*' => {
                self.advance();
                self.make_token(
                    TokenType::OperatorMultiply,
                    "*",
                    start_index,
                    start_line,
                    start_column,
                )
            }
            '/' => {
                self.advance();
                self.make_token(
                    TokenType::OperatorDivide,
                    "/",
                    start_index,
                    start_line,
                    start_column,
                )
            }
            // Compound operators
            '=' => {
                self.advance();
                if !self.is_at_end() && self.peek_char() == '=' {
                    self.advance();
                    self.make_token(
                        TokenType::OperatorEqual,
                        "==",
                        start_index,
                        start_line,
                        start_column,
                    )
                } else {
                    self.make_token(
                        TokenType::OperatorAssign,
                        "=",
                        start_index,
                        start_line,
                        start_column,
                    )
                }
            }
            '!' => {
                self.advance();
                if !self.is_at_end() && self.peek_char() == '=' {
                    self.advance();
                    self.make_token(
                        TokenType::OperatorNotEqual,
                        "!=",
                        start_index,
                        start_line,
                        start_column,
                    )
                } else {
                    self.make_token(TokenType::Not, "!", start_index, start_line, start_column)
                }
            }
            '<' => {
                self.advance();
                if !self.is_at_end() && self.peek_char() == '=' {
                    self.advance();
                    self.make_token(
                        TokenType::OperatorLessEq,
                        "<=",
                        start_index,
                        start_line,
                        start_column,
                    )
                } else if !self.is_at_end() && self.peek_char() == '>' {
                    self.advance();
                    self.make_token(
                        TokenType::OperatorNotEqual,
                        "<>",
                        start_index,
                        start_line,
                        start_column,
                    )
                } else {
                    self.make_token(
                        TokenType::OperatorLess,
                        "<",
                        start_index,
                        start_line,
                        start_column,
                    )
                }
            }
            '>' => {
                self.advance();
                if !self.is_at_end() && self.peek_char() == '=' {
                    self.advance();
                    self.make_token(
                        TokenType::OperatorGreaterEq,
                        ">=",
                        start_index,
                        start_line,
                        start_column,
                    )
                } else {
                    self.make_token(
                        TokenType::OperatorGreater,
                        ">",
                        start_index,
                        start_line,
                        start_column,
                    )
                }
            }
            // Strings
            '"' => self.scan_string(start_index, start_line, start_column),
            // Numbers
            c if c.is_ascii_digit() => {
                if let Some(token) = self.try_match_float(start_index, start_line, start_column) {
                    token
                } else if let Some(token) =
                    self.try_match_integer(start_index, start_line, start_column)
                {
                    token
                } else {
                    self.report_error(
                        "Invalid number".to_string(),
                        start_index,
                        start_line,
                        start_column,
                        ErrorType::InvalidNumber,
                    )
                }
            }
            // Identifiers & keywords
            c if c.is_alphabetic() || c == '_' => {
                if let Some(token) =
                    self.try_match_identifier(start_index, start_line, start_column)
                {
                    token
                } else {
                    self.report_error(
                        format!("Invalid identifier: '{}'", c),
                        start_index,
                        start_line,
                        start_column,
                        ErrorType::UnexpectedCharacter,
                    )
                }
            }
            // Unknown character
            c => {
                self.advance();
                self.report_error(
                    format!("Unknown character: '{}'", c),
                    start_index,
                    start_line,
                    start_column,
                    ErrorType::UnexpectedCharacter,
                )
            }
        }
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            match self.peek_char() {
                ' ' | '\t' | '\r' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.column = 0;
                    self.advance();
                }
                '/' => {
                    // Peek to see if it's a comment
                    if self.current_index + 1 < self.source.len()
                        && self.source[self.current_index + 1] == '/'
                    {
                        // Line comment
                        while !self.is_at_end() && self.peek_char() != '\n' {
                            self.advance();
                        }
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
    }

    fn scan_string(&mut self, start_index: usize, start_line: usize, start_column: usize) -> Token {
        self.advance(); // Consume "

        let mut value = String::new();

        while !self.is_at_end() && self.peek_char() != '"' {
            if self.peek_char() == '\\' {
                self.advance();

                if self.is_at_end() {
                    return self.report_error(
                        "Unterminated string".to_string(),
                        start_index,
                        start_line,
                        start_column,
                        ErrorType::UnterminatedString,
                    );
                }

                // Handle escape sequences
                match self.peek_char() {
                    'n' => value.push('\n'),
                    't' => value.push('\t'),
                    'r' => value.push('\r'),
                    '\\' => value.push('\\'),
                    '"' => value.push('"'),
                    '0' => value.push('\0'),
                    c => {
                        self.errors.push(LexerError::new(
                            format!("Invalid escape sequence '\\{}'", c),
                            Span::new(
                                self.current_index - 1,
                                self.current_index + 1,
                                self.line,
                                self.column - 1,
                            ),
                            ErrorType::InvalidEscape,
                        ));
                        value.push(c);
                    }
                }
                self.advance();
            } else if self.peek_char() == '\n' {
                // Multiline strings not allowed
                return self.report_error(
                    "Unterminated string before newline".to_string(),
                    start_index,
                    start_line,
                    start_column,
                    ErrorType::UnterminatedString,
                );
            } else {
                value.push(self.advance());
            }
        }

        if self.is_at_end() {
            return self.report_error(
                "Unterminated string (end of file)".to_string(),
                start_index,
                start_line,
                start_column,
                ErrorType::UnterminatedString,
            );
        }

        self.advance(); // Consume "

        Token::new(
            TokenType::StringLiteral,
            value,
            Span::new(start_index, self.current_index, start_line, start_column),
        )
    }

    fn make_token(
        &self,
        type_: TokenType,
        lexeme: &str,
        start_index: usize,
        start_line: usize,
        start_column: usize,
    ) -> Token {
        Token::new(
            type_,
            lexeme.to_string(),
            Span::new(start_index, self.current_index, start_line, start_column),
        )
    }

    fn report_error(
        &mut self,
        message: String,
        start_index: usize,
        start_line: usize,
        start_column: usize,
        error_type: ErrorType,
    ) -> Token {
        let span = Span::new(start_index, self.current_index, start_line, start_column);

        self.errors
            .push(LexerError::new(message.clone(), span, error_type));

        Token::error(message, span)
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current_index];
        self.current_index += 1;
        self.column += 1;
        c
    }

    fn peek_char(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current_index]
        }
    }

    fn is_at_end(&self) -> bool {
        self.current_index >= self.source.len()
    }

    fn try_match_identifier(
        &mut self,
        start_index: usize,
        start_line: usize,
        start_column: usize,
    ) -> Option<Token> {
        let save_index = self.current_index;

        let identifier_dfa = self.identifier_dfa.clone();
        let mut runner = DfaRunner::new(&identifier_dfa);
        let mut last_accept_index = None;

        while !self.is_at_end() {
            let c = self.peek_char();
            let result = runner.transition(c);

            match result {
                TransitionResult::Reject => break,
                TransitionResult::Accepted => {
                    self.advance();
                    last_accept_index = Some(self.current_index);
                }
                TransitionResult::Continue => {
                    self.advance();
                }
            }
        }

        if let Some(end_index) = last_accept_index {
            let lexeme: String = self.source[start_index..end_index].iter().collect();

            let token_type = self
                .keywords
                .get(&lexeme)
                .cloned()
                .unwrap_or(TokenType::Identifier);

            return Some(Token::new(
                token_type,
                lexeme,
                Span::new(start_index, end_index, start_line, start_column),
            ));
        }

        self.current_index = save_index;
        None
    }

    fn try_match_integer(
        &mut self,
        start_index: usize,
        start_line: usize,
        start_column: usize,
    ) -> Option<Token> {
        let save_index = self.current_index;

        let integer_dfa = self.integer_dfa.clone();
        let mut runner = DfaRunner::new(&integer_dfa);
        let mut last_accept_index = None;

        while !self.is_at_end() {
            let c = self.peek_char();
            let result = runner.transition(c);

            match result {
                TransitionResult::Reject => break,
                TransitionResult::Accepted => {
                    self.advance();
                    last_accept_index = Some(self.current_index);
                }
                TransitionResult::Continue => {
                    self.advance();
                }
            }
        }

        if let Some(end_index) = last_accept_index {
            let lexeme: String = self.source[start_index..end_index].iter().collect();
            return Some(Token::new(
                TokenType::Integer,
                lexeme,
                Span::new(start_index, end_index, start_line, start_column),
            ));
        }

        self.current_index = save_index;
        None
    }

    fn try_match_float(
        &mut self,
        start_index: usize,
        start_line: usize,
        start_column: usize,
    ) -> Option<Token> {
        let save_index = self.current_index;

        let float_dfa = self.float_dfa.clone();
        let mut runner = DfaRunner::new(&float_dfa);
        let mut last_accept_index = None;

        while !self.is_at_end() {
            let c = self.peek_char();
            let result = runner.transition(c);

            match result {
                TransitionResult::Reject => break,
                TransitionResult::Accepted => {
                    self.advance();
                    last_accept_index = Some(self.current_index);
                }
                TransitionResult::Continue => {
                    self.advance();
                }
            }
        }

        if let Some(end_index) = last_accept_index {
            let lexeme: String = self.source[start_index..end_index].iter().collect();
            return Some(Token::new(
                TokenType::Float,
                lexeme,
                Span::new(start_index, end_index, start_line, start_column),
            ));
        }

        self.current_index = save_index;
        None
    }

    fn load_identifier_dfa() -> Result<Automata, Box<dyn Error>> {
        let content = fs::read_to_string("src/lexer/dfas/identifier.dfa")
            .map_err(|e| format!("Error loading identifier DFA: {}", e))?;
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        Automata::from_lines(&lines)
    }

    fn load_integer_dfa() -> Result<Automata, Box<dyn Error>> {
        let content = fs::read_to_string("src/lexer/dfas/integer.dfa")
            .map_err(|e| format!("Error loading integer DFA: {}", e))?;
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        Automata::from_lines(&lines)
    }

    fn load_float_dfa() -> Result<Automata, Box<dyn Error>> {
        let content = fs::read_to_string("src/lexer/dfas/float.dfa")
            .map_err(|e| format!("Error loading float DFA: {}", e))?;
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        Automata::from_lines(&lines)
    }
}

// let tokens: Vec<Token> = vec![
//     Token { type_: TokenType::KeywordIf, lexeme: "if".to_string(), line: 1 },
//     Token { type_: TokenType::ParenthesisLeft, lexeme: "(".to_string(), line: 1 },
//     Token { type_: TokenType::Identifier, lexeme: "x".to_string(), line: 1 },
//     Token { type_: TokenType::OperatorEqual, lexeme: "==".to_string(), line: 1 },
//     Token { type_: TokenType::Number, lexeme: "5".to_string(), line: 1 },
// ];
