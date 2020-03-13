use std::io;
use std::result::Result;
use std::vec::Vec;

use compiler::tokeniser;
use compiler::Expr;

fn main() {
    loop {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Enter a lispcalc expression!");
        println!("{}", line);
        for token in tokeniser::tokenise(&line).unwrap().iter() {
            println!("{}", token);
        }
    }
}

fn read() -> Result<Expr, String> {
    unimplemented!();
}

fn eval(expr: Expr) -> Result<f64, String> {
    unimplemented!();
}

fn print(val: f64) {
    unimplemented!();
}
