use std::fmt::{Debug, Display};

use logos::{Lexer, Logos};

pub fn display<'a, T: Logos<'a> + Debug>(mut lex: Lexer<'a, T>) {
    let mut index = 0;

    let mut is_token = true;
    while is_token {
        let v = lex.next();
        println!("{:?}", v);
        if let Some(Ok(token)) = v {
            println!("{index} : {:?}", token);
            println!("{index} : {:?}", lex.slice());
            println!("{index} : {:?}", lex.span());
            println!();
            index += 1;
        } else {
            is_token = false;
            break;
        }
    }
}
