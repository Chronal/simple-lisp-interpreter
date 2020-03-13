mod eval;
mod parser;
pub mod tokeniser;

use std::fmt;

// An expression can be either an atom or a list of expressions
// Eval accepts an expression and returns a Value
pub enum Expr {
    Atom(f64),
    List(Vec<Expr>),
}

pub enum Token {
    LeftParen,
    RightParen,
    NumberCandidate(String),
    Add,
    Mult,
    Sub,
    Div,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Token::LeftParen => write!(f, "LEFT_PAREN"),
            Token::RightParen => write!(f, "RIGHT_PAREN"),
            Token::NumberCandidate(s) => write!(f, "{}", s),
            Token::Add => write!(f, "ADD"),
            Token::Mult => write!(f, "MULT"),
            Token::Sub => write!(f, "SUB"),
            Token::Div => write!(f, "DIV"),
        }
    }
}
