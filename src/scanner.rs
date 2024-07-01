use crate::token::{Token, TokenType, Object};

pub struct Scanner {
    source : String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    // Check if we are at the end of the source code
    fn is_at_end(self: &Self) -> bool{
        self.current >= self.source.len()
    }

    // Advance the current character
    fn advance(self: &mut Self) -> char{
        let character = self.source.chars().nth(self.current).unwrap(); 
        self.current += 1;
        character
    }

    // Appends a token to the vec of tokens
    fn add_token(self: &mut Self, token_type: TokenType) {
        self.add_token_lit(token_type,None)
    }

    // Another func for adding tokens where literal is also mentioned
    fn add_token_lit(self: &mut Self,token_type: TokenType, literal: Option<Object>) {
        let text = &self.source[self.start..self.current];
        let token_type = &token_type;
        let literal = &literal;
        self.tokens.push(Token{
            token_type: token_type.clone(),
            lexeme: text.to_string(),
            literal: literal.clone().unwrap_or(Object::IntVal(0)),
            line: self.line,
        });
    }

    // Scan for tokens 
    fn scan_token(self: &mut Self) -> Result<(),String>{
        let character = self.advance();
        match character{
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(crate::token::TokenType::RightParen),
            '{' => self.add_token(crate::token::TokenType::LeftBrace),
            '}' => self.add_token(crate::token::TokenType::RightBrace),
            ',' => self.add_token(crate::token::TokenType::Comma),
            '.' => self.add_token(crate::token::TokenType::Dot),
            '-' => self.add_token(crate::token::TokenType::Minus),
            '+' => self.add_token(crate::token::TokenType::Plus),
            ';' => self.add_token(crate::token::TokenType::Semicolon),
            '*' => self.add_token(crate::token::TokenType::Star),
            _ => Err(format!("Unexpected character {} at line {}",character,self.line))?,
        }
        Ok(())
    
    }

    // Initialize the scanner
    pub fn new(_source: &str) -> Self {
        Self {
            source: _source.to_string(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    // Scan tokens + put errors into a buffer
    pub fn scan_tokens(self: &mut Self) -> Result<Vec<Token>, String> {
        // Error buffer
        let mut errors = Vec::new();
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token(){
                Ok(_) => {},
                Err(e) => errors.push(e),
            }
        }

        self.tokens.push(Token{
            token_type: crate::token::TokenType::Eof,
            lexeme: "".to_string(),
            literal: crate::token::Object::IntVal(0),
            line: self.line,
        });

        // If errors do exist then map each error and join for nice error checking
        if errors.len() > 0{
            let mut joined = String::new();
            errors.iter().for_each(|e| {
                joined.push_str(e);
                joined.push_str("\n");
            });
            return Err(joined);
        }

        Ok(self.tokens.clone())
        
    }
}
