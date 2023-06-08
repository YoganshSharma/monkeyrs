use crate::lexer::TokenType;
struct Program<T: Statement> {
    statements: Vec<T>,
}

pub enum StatementType {
    LetStatement,
    ReturnStatement,
}

trait Statement {
    fn get_statement_type(&self) -> StatementType {
        StatementType::LetStatement
    }
}

trait Expression {}

struct LetStatement<T: Expression> {
    ident: TokenType,
    value: T,
}
