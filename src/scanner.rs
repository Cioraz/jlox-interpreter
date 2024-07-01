// Scanner module for scanning the source code and converting it into tokens
use crate::token::{Object, Token, TokenType};

// Scanner struct
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    // Check if we are at the end of the source code
    fn is_at_end(self: &Self) -> bool {
        self.current >= self.source.len()
    }

    // Advance the current character
    fn advance(self: &mut Self) -> char {
        let character = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        character
    }

    // Appends a token to the vec of tokens
    fn add_token(self: &mut Self, token_type: TokenType) {
        self.add_token_lit(token_type, None)
    }

    // Another func for adding tokens where literal is also mentioned
    fn add_token_lit(self: &mut Self, token_type: TokenType, literal: Option<Object>) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token {
            token_type: token_type.clone(),
            lexeme: text.to_string(),
            literal: literal.clone().unwrap_or(Object::IntVal(0)),
            line: self.line,
        });
    }

    fn peek(self: &Self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    // Scan for tokens
    fn scan_token(self: &mut Self) -> Result<(), String> {
        let character = self.advance();
        match character {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            ' ' => {},
            '\r' => {},
            '\t' => {},
            '\n' => self.line += 1,
            '!' => {
                if self.char_match('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.char_match('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '<' => {
                if self.char_match('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            '>' => {
                if self.char_match('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }
            '/' => {
                if self.char_match('/') {
                    loop{
                        if self.peek() == '\n' || self.is_at_end() {
                            break;
                        }
                        self.advance();
                    }
                    
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            _ => Err(format!(
                "Unexpected character {} at line {}",
                character, self.line
            ))?,
        }
        Ok(())
    }

    fn char_match(self: &mut Self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        true
    }

    // Initialize the scanner
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
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
            match self.scan_token() {
                Ok(_) => {}
                Err(e) => errors.push(e),
            }
        }

        self.tokens.push(Token {
            token_type: crate::token::TokenType::Eof,
            lexeme: "".to_string(),
            literal: crate::token::Object::IntVal(0),
            line: self.line,
        });

        // If errors do exist then map each error and join for nice error checking
        if errors.len() > 0 {
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_char_token() {
        let source = "(),.-+;*";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 9);
        assert_eq!(tokens[0].token_type,TokenType::LeftParen);
        assert_eq!(tokens[1].token_type,TokenType::RightParen);
        assert_eq!(tokens[2].token_type,TokenType::Comma);
        assert_eq!(tokens[3].token_type,TokenType::Dot);
        assert_eq!(tokens[4].token_type,TokenType::Minus);
        assert_eq!(tokens[5].token_type,TokenType::Plus);
        assert_eq!(tokens[6].token_type,TokenType::Semicolon);
        assert_eq!(tokens[7].token_type,TokenType::Star);
        assert_eq!(tokens[8].token_type,TokenType::Eof);
    }

    #[test]
    fn operators(){
        let source = "!= >= <= ==";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        assert_eq!(tokens.len(),5);
        assert_eq!(tokens[0].token_type,TokenType::BangEqual);
        assert_eq!(tokens[1].token_type,TokenType::GreaterEqual);
        assert_eq!(tokens[2].token_type,TokenType::LessEqual);
        assert_eq!(tokens[3].token_type,TokenType::EqualEqual);
        assert_eq!(tokens[4].token_type,TokenType::Eof);

    }
}