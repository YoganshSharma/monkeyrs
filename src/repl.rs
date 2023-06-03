use std::io::{BufReader, BufWriter, Write, BufRead, Read};
use anyhow::Result;
use crate::lexer::{Lexer,TokenType};

const PROOMPT: &str = ">>";

pub fn start<R: Read,W: Write>(mut reader: BufReader<R>, mut writer:BufWriter<W>) -> Result<()> {
    loop {
        write!(writer,"{}", PROOMPT)?;
        writer.flush()?;
        let mut buffer = String::new();
        
        BufReader::read_line(&mut reader, &mut buffer)?;
        let mut lex = Lexer::new(buffer);
        while let Ok(tok) = lex.next_token() {
            if tok == TokenType::Eof {
                break;
            }
            writeln!(writer, "{:?}", tok )?;
        }
        writer.flush()?;
    }

}
