use crate::token_type::*;
use std::fmt::{self};

#[derive(Debug)]
pub enum Object {
    Num(i64),
    Str(String),
    Nil,
    True,
    False,
}
impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Num(x) => write!(f, "{x}"),
            Object::Str(x) => write!(f, "\"{x}\""),
            Object::Nil => write!(f, "nil"),
            Object::True => write!(f, "True"),
            Object::False => write!(f, "False"),
        }
    }
}

#[derive(Debug)]
pub struct Token {
    ttype: TokenType,
    lexeme: String,
    literal: Option<Object>,
    line: usize,
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, literal: Option<Object>, line: usize) -> Token {
        Token {
            ttype,
            lexeme,
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} {} {}",
            self.ttype,
            self.lexeme,
            if let Some(literal) = self.literal {
                literal.to_string()
            } else {
                "None".to_string()
            }
        )
    }
}

// pub enum Token {
//     Literal {lemexe:String, literal: <..>},
//     Keyword {lemexe:String, ttype: String,}
// }
