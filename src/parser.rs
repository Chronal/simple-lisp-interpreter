use crate::Expr;
use std::result::Result;

fn parse() -> Result<Expr, String> {
    todo!();
}

fn string_to_f64(num_candidate: &str) -> Result<f64, String> {
    let list: Vec<&str> = num_candidate.split('.').collect();
    let len = list.len();

    if (len > 2) {
        return Result::Err(String::from("Too many decimal points in number literal"));
    } else {
        return Result::Ok(num_candidate.parse::<f64>().unwrap());
    }
}
