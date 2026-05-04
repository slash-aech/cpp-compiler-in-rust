use crate::token::{Token,TokenKind};

pub struct Lexer{
    source: Vec<char>,
    token: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
    errors: Vec<LexerError>,
}

#[derive(Clone)]
pub struct LexerError{
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl Lexer{
    pub fn new(source: &str)->Self{
        Lexer{
            source: source.chars().collect(),
            token: Vec::new(),
            start:0,
            current:0,
            line:1,
            column:1,
            errors: Vec::new(),
        }
    }

    pub fn tokenize(&mut self)->Result<Vec<Token>, Vec<LexerError>>{
        while !self.is_at_end(){
            self.start=self.current;
            self.scan_token();
        }
        self.token.push(Token::new(
            TokenKind::EOF,
            String::new(),
            self.line,
            self.column,
        )
        );
        if self.errors.is_empty(){
            Ok(self.token.clone())
        }
        else{
            Err(self.errors.clone())
        }
    }
    pub fn scan_token(&mut self){
        let c = self.advance();
        match c{
            //single char tokens
            '('=>self.add_token(TokenKind::LeftParen),
            ')'=>self.add_token(TokenKind::RightParen),
            '{'=>self.add_token(TokenKind::LeftBrace),
            '}'=>self.add_token(TokenKind::RightBrace),
            ':'=>self.add_token(TokenKind::Colon),
            ';'=>self.add_token(TokenKind::SemiColon),
            ','=>self.add_token(TokenKind::Comma),
            '+'=>self.add_token(TokenKind::Plus),
            '*'=>self.add_token(TokenKind::Star),
            '%'=>self.add_token(TokenKind::Percent),

            //single or multi until understood
            '-'=>{
                if self.match_char('>'){
                    self.add_token(TokenKind::Arrow)
                }
                else{
                    self.add_token(TokenKind::Minus)
                }
            },
            '/'=>{
                if self.match_char('/'){
                    while self.peek()!='\n'&& !self.is_at_end(){
                        self.advance();
                    }
                }
                else{
                    self.add_token(TokenKind::Slash)
                }
            },
            '='=>{
                if self.match_char('='){
                    self.add_token(TokenKind::EqualEqual)
                }
                else{
                    self.add_token(TokenKind::Equal)
                }
            },
            '!'=>{
                if self.match_char('='){
                    self.add_token(TokenKind::BangEqual)
                }
                else{
                    self.add_token(TokenKind::Bang)
                }
            },
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenKind::LessEqual);
                } else {
                    self.add_token(TokenKind::Less);
                }
            },
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenKind::GreaterEqual);
                } else {
                    self.add_token(TokenKind::Greater);
                }
            },
            '&'=>{
                if self.match_char('&'){
                    self.add_token(TokenKind::And)
                }
                else{
                    self.error("Expected && for logical AND")
                }
            },
            '|'=>{
                if self.match_char('|'){
                    self.add_token(TokenKind::Or)
                }
                else{
                    self.error("Expected || for logical Or")
                }
            },

            //whitespaces and shit
            ' '|'\r'|'\t'=>{}
            '\n'=>{
                self.line += 1;
                self.column = 1;
            },

            //Strings and stuff yk
            '"'=>self.string(),

            // numbers and identifiers
            _ => {
                if c.is_ascii_digit(){
                    self.number();
                }
                else if c.is_alphabetic() || c == '_'{
                    self.identifier();
                }
                else{
                    self.error(&format!("Unexpected character-> {}",c));
                }
            }
        }
    }


    fn string(&mut self){
        let start_line = self.line;
        let start_column = self.column;
        while self.peek()!='"'&&!self.is_at_end(){
            if self.peek()=='\n'{
                self.line+=1;
                self.column=1;
            }
            self.advance();
        }
        if self.is_at_end(){
            self.errors.push(LexerError{
                message: "Unterminated string".to_string(),
                line: start_line,
                column: start_column,
            });
            return;
        }
        self.advance(); //this should eat the final quote
        let value : String = self.source[self.start+1..self.current-1]
        .iter()
        .collect();
        self.add_token(TokenKind::StringLiteral(value));
    }
    fn number(&mut self){
        while self.peek().is_ascii_digit(){
            self.advance();
        }
        if self.peek()=='.' && self.peek_next().is_ascii_digit(){
            self.advance();//DECIMAL point is consumed
            while self.peek().is_ascii_digit(){ 
                self.advance();
            }
            let value:String = self.source[self.start..self.current].iter().collect();
            let float_val:f64 = value.parse().unwrap();
            self.add_token(TokenKind::FloatLiteral(float_val));
        }
        else{
            while self.peek().is_ascii_digit(){
                self.advance();
            }
            let value:String = self.source[self.start..self.current].iter().collect();
            let int_val:i64 = value.parse().unwrap();
            self.add_token(TokenKind::IntegerLiteral(int_val));
        }
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let text: String = self.source[self.start..self.current].iter().collect();

        // Check for keywords
        let kind = match text.as_str() {
            "fn" => TokenKind::Function,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "return" => TokenKind::Return,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "int" => TokenKind::Integer,
            "float" => TokenKind::Float,
            "bool" => TokenKind::Bool,
            "string" => TokenKind::String,
            _ => TokenKind::Identifier(text.clone()),
        };

        self.add_token(kind);
    }

    // Helper methods

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        self.column += 1;
        c
    }

    fn peek(&self) -> char {
        if self.is_at_end() { '\0' } else { self.source[self.current] }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            return false;
        }
        self.current += 1;
        self.column += 1;
        true
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn add_token(&mut self, kind: TokenKind) {
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        self.token.push(Token::new(kind, lexeme, self.line, self.column));
    }

    fn error(&mut self, message: &str) {
        self.errors.push(LexerError {
            message: message.to_string(),
            line: self.line,
            column: self.column,
        });
    }
}