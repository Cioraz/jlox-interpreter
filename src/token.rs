#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals,
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof
}

#[derive(Debug)]
pub enum Object{
    IntVal(i64),
    FloatVal(f64),
    StringVal(String),
    IdentifierVal(String),
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Object,
    line: u64, 
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Object, line: u64) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}