use std::result::Result;
use std::vec::Vec;

use crate::Expr;
use crate::Token;

fn is_ascii(byte: u8) -> bool {
    // Only need the first bit of the byte to check if the byte is part of a utf-8 character
    let bitmask = 0b10000000;
    let is_ascii = (byte & bitmask) == 0;
    return is_ascii;
}

fn valid_char(byte: u8) -> bool {
    is_digit(byte) || is_dot(byte) || is_whitespace(byte) || is_parens(byte)
}

fn is_digit(byte: u8) -> bool {
    // '0' - '9'
    byte >= 48 && byte < 57
}

fn is_dot(byte: u8) -> bool {
    // '.'
    byte == 46
}

fn is_whitespace(byte: u8) -> bool {
    // spaces, tab, newlines
    byte == 32 || byte == 9 || byte == 10
}

fn is_parens(byte: u8) -> bool {
    // '(', ')'
    byte == 40 || byte == 41
}

fn is_op(byte: u8) -> bool {
    // '+', '-', '*', '/'
    byte == 43 || byte == 45 || byte == 42 || byte == 47
}

pub fn tokenise(src_code: &str) -> Result<Vec<Token>, String> {
    let mut token_list: Vec<Token> = Vec::new();

    let bytes = src_code.as_bytes();
    let mut num_token = String::new();
    let mut in_num = false;

    for index in 0..bytes.len() {
        let character = bytes[index];
        if !is_ascii(character) {
            return Result::Err(String::from(
                "Not a valid ascii character. utf-8 characters are not cuurently supported",
            ));
        }

        if is_whitespace(character) {
            if in_num {
                in_num = false;
                token_list.push(Token::NumberCandidate(num_token));
                num_token = String::new();
            }
        } else if is_parens(character) {
            if in_num {
                in_num = false;
                token_list.push(Token::NumberCandidate(num_token));
                num_token = String::new();
            }

            match character {
                40 => token_list.push(Token::LeftParen),
                41 => token_list.push(Token::RightParen),
                _ => continue,
            }
        } else if is_digit(character) {
            if !in_num {
                in_num = true;
            }
            num_token.push(char::from(character));
        } else if is_dot(character) {
            if !in_num {
                return Result::Err(String::from("Dot character without preceding number. Zero must be put explicitly for numbers less than 1"));
            } else {
                num_token.push(char::from(character));
            }
        } else if is_op(character) {
            if in_num {
                in_num = false;
                token_list.push(Token::NumberCandidate(num_token));
                num_token = String::new();
            }

            let token = match character {
                43 => Token::Add,
                45 => Token::Sub,
                42 => Token::Mult,
                47 => Token::Div,
                _ => continue,
            };

            token_list.push(token);
        } else {
            return Result::Err(String::from("Invalid characters used."));
        }
    }

    return Result::Ok(token_list);
}
