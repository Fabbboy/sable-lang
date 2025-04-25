use crate::position::Position;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum TokenType {
    #[default]
    Eof,
    Err,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Token<'s> {
    pub token_type: TokenType,
    pub lexeme: &'s str,
    pub pos: Position,
}

impl<'s> Token<'s> {
    pub fn new(token_type: TokenType, lexeme: &'s str, pos: Position) -> Self {
        Self {
            token_type,
            lexeme,
            pos,
        }
    }
}
