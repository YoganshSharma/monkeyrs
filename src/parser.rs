use crate::lexer::{Lexer, TokenType};
use anyhow::Result;

struct Parser {
    lexer: Lexer,
    cur_token: TokenType,
    peek_token: TokenType,
}

impl Parser {
    fn new(lexer: Lexer) -> Result<Self> {
        let mut p = Parser {
            lexer,
            cur_token: TokenType::Illegal,
            peek_token: TokenType::Illegal,
        };
        p.next_token()?;
        p.next_token()?;
        Ok(p)
    }

    fn next_token(&mut self) -> Result<()> {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token()?;
        Ok(())
    }
}
