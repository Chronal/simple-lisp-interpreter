extern crate regex;

use std::io;
use std::result::Result;
use std::collections::HashMap;
use std::vec::Vec;

#[allow(unused_imports)]
use regex::Regex;

// An atom is either a symbol or a number (maybe add strings later)
// Symbols are generally just strings, functions/variables can be bound to them
enum Atom {
    Num(f64),
    Symbol(String)
}

// An expression can be either an atom or a list of expressions
// Eval accepts an expression and returns a Value
enum Expr {
    Atom(Atom),
    List(Vec<Expr>),
}

// A value can be either an atom, list of values or a function
enum Value {
    Atom(Atom),
    List(Vec<Value>),
    Function(Box<Fn(Value) -> Value>),
}

fn main() {
    let mut env = gen_env();

    // REPL
    loop {
        let exp = match read() {
            Ok(read_exp) => read_exp,
            Err(e) => {
                eprintln!("{}",e);
                continue
            },
        };

        match eval(exp,&mut env) {
            Ok(val) => print(val),
            Err(e) => {
                eprintln!("{}",e);
            }
        }
    }
}

fn read() -> Result<Expr,String> {
    unimplemented!();
}

fn eval(expr: Expr, env: &mut HashMap<String,Value>) -> Result<Value,String> {
    unimplemented!();
}

fn print(val : Value) {
    unimplemented!();
}

fn gen_env() -> HashMap<String,Value> {
    unimplemented!();
}
