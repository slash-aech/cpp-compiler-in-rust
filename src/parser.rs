use crate::token::*;
use crate::ast::*;

pub struct Parser{
    tokens: Vec<Token>,
    pos: usize,
}

pub fn new(tokens: Vec<Token>) -> Self{
    Self{tokens, pos:0}
}
fn peek(&self) -> Token{
    &self.tokens[self.pos]
}