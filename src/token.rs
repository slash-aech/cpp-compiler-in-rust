#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    //Literals
    IntegerLiteral(i64),
    FloatLiteral(f64),
    Identifier(String),
    StringLiteral(String),
    BoolLiteral(bool),

    //these are literal keywords, upper are the values
    Integer,
    Float,
    Bool,
    String,

    //Keywords
    Function,
    If,
    Else,
    Return,
    True,
    False,

    //Operators
    Plus,
    Minus,
    Slash,
    Star,
    Percent,
    Equal,
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
    Bang,

    //Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    SemiColon,
    Colon,
    Comma,
    Arrow,

    //Special
    EOF,
}
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, line: usize, column: usize) -> Self {
        Token {
            kind,
            lexeme,
            line,
            column,
        }
    }
}
