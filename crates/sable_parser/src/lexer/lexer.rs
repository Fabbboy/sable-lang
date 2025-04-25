use crate::position::{Position, Range};

use super::token::{Token, TokenType};

pub struct Lexer<'s> {
    source: &'s str,

    // Position
    current: usize,
    start: usize,
    line: usize,
    column: usize,

    // Tokens
    curr_tok: Token<'s>,
    next_tok: Token<'s>,
}

impl<'s> Lexer<'s> {
    pub fn new(source: &'s str) -> Self {
        let mut l = Self {
            source,
            current: 0,
            start: 0,
            line: 1,
            column: 1,
            curr_tok: Token::default(),
            next_tok: Token::default(),
        };
        l.lex();
        l
    }

    fn get_token(&self, token_type: TokenType) -> Token<'s> {
        Token::new(token_type, self.get_lexeme(), self.get_pos())
    }

    fn get_lexeme(&self) -> &'s str {
        if self.current > self.start {
            &self.source[self.start..self.current]
        } else {
            ""
        }
    }

    fn get_pos(&self) -> Position {
        Position::new(self.line, self.column, Range::new(self.start, self.current))
    }

    fn get_char(&self) -> Option<char> {
        if self.current < self.source.len() {
            self.source[self.current..].chars().next()
        } else {
            None
        }
    }

    fn lex_trivial(&mut self) {
        let c = self.get_char().unwrap_or('\0');
        match c {
            ' ' | '\r' | '\t' => {
                self.current += 1;
                self.column += 1;
            }
            '\n' => {
                self.line += 1;
                self.column = 1;
                self.current += 1;
            }
            _ => {}
        }
    }

    fn next(&mut self) -> Token<'s> {
        self.lex_trivial();
        self.start = self.current;
        let c = self.get_char();
        if c.is_none() {
            return self.get_token(TokenType::Eof);
        }
        let c = c.unwrap();
        self.current += 1;

        match c {
            '\0' => self.get_token(TokenType::Eof),
            _ => return self.get_token(TokenType::Err),
        }
    }

    pub fn lex(&mut self) -> Token<'s> {
        self.curr_tok = self.next_tok.clone();
        self.next_tok = self.next();
        self.curr_tok.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::position::Range;

    #[test]
    fn test_basic_lexer() {
        let source = "";
        let mut lexer = Lexer::new(source);

        let token = lexer.lex();
        assert_eq!(token.token_type, TokenType::Eof);
        assert_eq!(token.lexeme, "");
        assert_eq!(token.pos.line, 1);
        assert_eq!(token.pos.column, 1);
        assert_eq!(token.pos.start, Range::new(0, 0));
    }
}
