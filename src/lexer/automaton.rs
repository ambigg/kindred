// use crate::lexer::helper::automaton_helper;
use std::{self, collections::HashMap, error::Error, fmt};

#[derive(Debug, Clone)]
pub struct Automata {
    pub alphabet: Vec<char>,
    pub states: Vec<String>,
    pub initial_state: String,
    pub final_states: Vec<String>,
    transitions: HashMap<(String, char), String>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TransitionResult {
    Accepted,
    Continue,
    Reject,
}

#[derive(Debug)]
pub enum DfaError {
    InvalidInitialState(String),
    InvalidState(String),
    InvalidSymbol(char),
    DuplicateTransition(String, char),
    InvalidFormat(String),
}

impl fmt::Display for DfaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DfaError::InvalidInitialState(s) => {
                write!(f, "Initial state '{}' does not exist", s)
            }
            DfaError::InvalidState(s) => write!(f, "State '{}' is not valid", s),
            DfaError::InvalidSymbol(c) => {
                write!(f, "Symbol '{}' does not exist in the alphabet", c)
            }
            DfaError::DuplicateTransition(s, c) => {
                write!(f, "Duplicate transition ({}, '{}')", s, c)
            }
            DfaError::InvalidFormat(msg) => write!(f, "Invalid Format: {}", msg),
        }
    }
}

impl Error for DfaError {}

pub struct DfaRunner<'a> {
    automata: &'a Automata,
    current_state: String,
}

impl Automata {
    pub fn create_runner(&self) -> DfaRunner<'_> {
        DfaRunner::new(self)
    }

    pub fn from_lines(lines: &[String]) -> Result<Self, Box<dyn Error>> {
        if lines.len() < 4 {
            return Err(DfaError::InvalidFormat(
                "The format needs to be more than 4 lines".to_string(),
            )
            .into());
        }
        let alphabet = Self::parse_alphabet(&lines[0])?;

        let states: Vec<String> = lines[1]
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        if states.is_empty() {
            return Err(DfaError::InvalidFormat("No states defined".to_string()).into());
        }

        let initial_state = lines[2]
            .split(',')
            .next()
            .ok_or_else(|| DfaError::InvalidFormat("There is no initial state".to_string()))?
            .trim()
            .to_string();

        if !states.contains(&initial_state) {
            return Err(DfaError::InvalidInitialState(initial_state).into());
        }

        let final_states: Vec<String> = lines[3]
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        for final_state in &final_states {
            if !states.contains(final_state) {
                return Err(DfaError::InvalidState(final_state.clone()).into());
            }
        }

        let transitions = Self::parse_transitions(&lines[4..], &states, &alphabet)?;

        Ok(Automata {
            alphabet,
            states,
            initial_state,
            final_states,
            transitions,
        })
    }

    fn parse_alphabet(line: &str) -> Result<Vec<char>, Box<dyn Error>> {
        let symbols: Vec<String> = line
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let mut expanded_alphabet: Vec<char> = Vec::new();

        for symbol in symbols {
            match symbol.as_str() {
                "$" => {
                    // Digits 0-9
                    expanded_alphabet.extend('0'..='9');
                }
                "%" => {
                    // Letters a-z, A-Z
                    expanded_alphabet.extend('a'..='z');
                    expanded_alphabet.extend('A'..='Z');
                }
                "&" => {
                    // Digits & letters
                    expanded_alphabet.extend('0'..='9');
                    expanded_alphabet.extend('a'..='z');
                    expanded_alphabet.extend('A'..='Z');
                }
                _ => {
                    if symbol.len() == 1 {
                        expanded_alphabet.push(symbol.chars().next().unwrap());
                    } else {
                        return Err(DfaError::InvalidFormat(format!(
                            "The symbol '{}' must be a valid character",
                            symbol
                        ))
                        .into());
                    }
                }
            }
        }

        expanded_alphabet.sort_unstable();
        expanded_alphabet.dedup();

        Ok(expanded_alphabet)
    }

    fn parse_transitions(
        lines: &[String],
        states: &[String],
        alphabet: &[char],
    ) -> Result<HashMap<(String, char), String>, Box<dyn Error>> {
        let mut transitions = HashMap::new();

        for line in lines {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split(&['=', ','][..]).map(|s| s.trim()).collect();

            if parts.len() != 3 {
                return Err(DfaError::InvalidFormat(format!("Invalid line: '{}'", line)).into());
            }

            let origin_state = parts[0].to_string();
            let symbol_str = parts[1];
            let destination_state = parts[2].to_string();

            if !states.contains(&origin_state) {
                return Err(DfaError::InvalidState(origin_state).into());
            }
            if !states.contains(&destination_state) {
                return Err(DfaError::InvalidState(destination_state).into());
            }

            if symbol_str.len() != 1 {
                return Err(DfaError::InvalidFormat(format!(
                    "Symbol '{}' must be only one character",
                    symbol_str
                ))
                .into());
            }
            let symbol = symbol_str.chars().next().unwrap();

            if !alphabet.contains(&symbol) {
                return Err(DfaError::InvalidSymbol(symbol).into());
            }

            let key = (origin_state.clone(), symbol);
            if transitions.contains_key(&key) {
                return Err(DfaError::DuplicateTransition(origin_state, symbol).into());
            }
            transitions.insert(key, destination_state);
        }
        Ok(transitions)
    }

    pub fn initial_state(&self) -> &str {
        &self.initial_state
    }

    /// Checks if a state is final
    pub fn is_final_state(&self, state: &str) -> bool {
        self.final_states.contains(&state.to_string())
    }

    pub fn transition(&self, state: &str, symbol: char) -> Option<&String> {
        self.transitions.get(&(state.to_string(), symbol))
    }
}

impl<'a> DfaRunner<'a> {
    pub fn new(automata: &'a Automata) -> Self {
        let current_state = automata.initial_state().to_string();
        println!("DFA Runner started on state: {}", current_state);

        DfaRunner {
            automata,
            current_state,
        }
    }

    pub fn transition(&mut self, character: char) -> TransitionResult {
        match self.automata.transition(&self.current_state, character) {
            Some(next_state) => {
                self.current_state = next_state.clone();

                let result = if self.automata.is_final_state(&self.current_state) {
                    TransitionResult::Accepted
                } else {
                    TransitionResult::Continue
                };
                println!(
                    "  '{}' -> State: {} ({:?})",
                    character, self.current_state, result
                );
                result
            }

            None => {
                println!("  '{}' -> FAILED (no transition)", character);
                TransitionResult::Reject
            }
        }
    }

    pub fn reset(&mut self) {
        self.current_state = self.automata.initial_state().to_string();
    }

    /// Checks if it's on a final state
    pub fn is_on_final_state(&self) -> bool {
        self.automata.is_final_state(&self.current_state)
    }

    pub fn get_current_state(&self) -> &str {
        &self.current_state
    }

    pub fn process_string(&mut self, input: &str) -> bool {
        self.reset();
        println!("\nProcessing: '{}'", input);

        for c in input.chars() {
            let result = self.transition(c);
            if result == TransitionResult::Reject {
                return false;
            }
        }

        self.is_on_final_state()
    }
}
