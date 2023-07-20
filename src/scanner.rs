use crate::{error::LoxError, token::Token};

pub struct Scanner {
    pub source: String,
    tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
        }
    }
    pub fn scan_tokens(&self) -> Result<Vec<Token>, LoxError> {
        Ok(Vec::new())
    }
}
