use anyhow::Result;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    Ident(String),
    Int(String),

    Illegal,
    Eof,

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanEq,
    LessThanEq,
    Slash,

    // delimeters
    Comma,
    Semicolon,

    Lparen,
    Rparen,
    Lsquigly,
    Rsquigly,

    // keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,

}

pub struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Lexer {
            input: input.into_bytes(),
            position: 0,
            read_position: 0,
            ch: 0,
        };
        l.read_char();
        l
    }

    pub fn next_token(&mut self) -> Result<TokenType> {
        self.skip_whitespace();
        let tok = match self.ch {
            b';' => {
                Ok(TokenType::Semicolon)
            }
            b')' => {
                Ok(TokenType::Rparen)
            }
            b'(' => {
                Ok(TokenType::Lparen)
            }
            b'{' => {
                Ok(TokenType::Lsquigly)
            }
            b'}' => {
                Ok(TokenType::Rsquigly)
            }
            b',' => {
                Ok(TokenType::Comma)
            }
            b'+' => {
                Ok(TokenType::Plus)
            }
            b'-' => {
                Ok(TokenType::Minus)
            }
            b'*' => {
                Ok(TokenType::Asterisk)
            }
            b'/' => {
                Ok(TokenType::Slash)
            }
            b'!' => {
                match self.peek_char() {
                    b'=' => {
                        self.read_char();
                        Ok(TokenType::NotEqual)
                    }
                    _ => {
                        Ok(TokenType::Bang)
                    }
                }
            }
            b'<' => {
                match self.peek_char() {
                    b'=' => {
                        self.read_char();
                        Ok(TokenType::LessThanEq)
                    }
                    _ => {
                        Ok(TokenType::LessThan)
                    }
                }
            }
            b'>' => {
                match self.peek_char() {
                    b'=' => {
                        self.read_char();
                        Ok(TokenType::GreaterThanEq)
                    }
                    _ => {
                        Ok(TokenType::GreaterThan)
                    }
                }
            }
            b'=' => {
                match self.peek_char() {
                    b'=' => {
                        self.read_char();
                        Ok(TokenType::Equal)
                    }
                    _ => {
                        Ok(TokenType::Assign)
                    }
                }
            }
            b'a'..=b'z'|b'A'..=b'Z'|b'_' => {
                let ident = self.read_ident();
               return match ident.as_str() {
                    "let" => Ok(TokenType::Let),
                    "fn" => Ok(TokenType::Function),
                    "true" => Ok(TokenType::True),
                    "false" => Ok(TokenType::False),
                    "if" => Ok(TokenType::If),
                    "else" => Ok(TokenType::Else),
                    "return" => Ok(TokenType::Return),
                    _ => Ok(TokenType::Ident(ident)),
                }

            }
            b'0'..=b'9' => {
                return Ok(TokenType::Int(self.read_number()))
            }


            0 => Ok(TokenType::Eof),
            _ => {
                Ok(TokenType::Illegal)
            }
        };
        self.read_char();
        tok
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
             return 0;
        }
        self.input[self.read_position]
    }
    fn read_ident(&mut self) -> String {
        let mut literal = String::new();
        while self.is_ident_ch() {
            literal.push(self.ch.try_into().unwrap());
            self.read_char();
        } 
        literal
    }

    fn is_ident_ch(&self) -> bool {
        self.ch.is_ascii_alphabetic() || self.ch == b'_'
    }
    fn is_number_ch(&self) -> bool {
        self.ch.is_ascii_digit()
    }
    fn skip_whitespace(&mut self) {
        while matches!(self.ch, b' '| b'\t' |b'\n' |b'\r' ) {
           self.read_char() ;
        }
    }

    fn read_number(&mut self) -> String {
        let mut literal = String::new();
        while self.is_number_ch() {
            literal.push(self.ch.try_into().unwrap());
            self.read_char();
        } 
        literal
    }

}

#[test]
fn lexertest() {
    let input = String::from("let five = 5;
    let ten = 10;
    let add = fn(x, y) {
    x + y;
    };
    let result = add(five, ten);
    !-/*5;
    5 < 10 > 5;
    if (5 < 10) {
    return true;
    } else {
    return false;
    }
    10 == 10;
    10 != 9; ");
    let tests = vec![
        TokenType::Let,
        TokenType::Ident(String::from("five")),
        TokenType::Assign,
        TokenType::Int(String::from("5")),
        TokenType::Semicolon,
        TokenType::Let,
        TokenType::Ident(String::from("ten")),
        TokenType::Assign,
        TokenType::Int(String::from("10")),
        TokenType::Semicolon,
        TokenType::Let,
        TokenType::Ident(String::from("add")),
        TokenType::Assign,
        TokenType::Function,
        TokenType::Lparen,
        TokenType::Ident(String::from("x")),
        TokenType::Comma,
        TokenType::Ident(String::from("y")),
        TokenType::Rparen,
        TokenType::Lsquigly,
        TokenType::Ident(String::from("x")),
        TokenType::Plus,
        TokenType::Ident(String::from("y")),
        TokenType::Semicolon,
        TokenType::Rsquigly,
        TokenType::Semicolon,
        TokenType::Let,
        TokenType::Ident(String::from("result")),
        TokenType::Assign,
        TokenType::Ident(String::from("add")),
        TokenType::Lparen,
        TokenType::Ident(String::from("five")),
        TokenType::Comma,
        TokenType::Ident(String::from("ten")),
        TokenType::Rparen,
        TokenType::Semicolon,
        TokenType::Bang,
        TokenType::Minus,
        TokenType::Slash,
        TokenType::Asterisk,
        TokenType::Int(String::from("5")),
        TokenType::Semicolon,
        TokenType::Int(String::from("5")),
        TokenType::LessThan,
        TokenType::Int(String::from("10")),
        TokenType::GreaterThan,
        TokenType::Int(String::from("5")),
        TokenType::Semicolon,
        TokenType::If,
        TokenType::Lparen,
        TokenType::Int(String::from("5")),
        TokenType::LessThan,
        TokenType::Int(String::from("10")),
        TokenType::Rparen,
        TokenType::Lsquigly,
        TokenType::Return,
        TokenType::True,
        TokenType::Semicolon,
        TokenType::Rsquigly,
        TokenType::Else,
        TokenType::Lsquigly,
        TokenType::Return,
        TokenType::False,
        TokenType::Semicolon,
        TokenType::Rsquigly,
        TokenType::Int(String::from("10")),
        TokenType::Equal,
        TokenType::Int(String::from("10")),
        TokenType::Semicolon,
        TokenType::Int(String::from("10")),
        TokenType::NotEqual,
        TokenType::Int(String::from("9")),
        TokenType::Semicolon,
    ];
    let mut lex = Lexer::new(input);

    for tok in tests {
        assert_eq!(tok, lex.next_token().unwrap());
    }

}
