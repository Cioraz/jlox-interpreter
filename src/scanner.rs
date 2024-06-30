use crate::token::Token;

pub struct Scanner{

}

impl Scanner{
    pub fn new(_source: &str) -> Self{
        Self{}
    }

    pub fn scan_tokens(self:&Self) -> Result<Vec<Token>,String>{
        todo!("Implement the scan_tokens function in the Scanner struct");
    }
}