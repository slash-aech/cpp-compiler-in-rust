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

pub struct LexerError{
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl Lexer{
    pub fn new(source: &str)->self{
        Lexer{
            source: source.char().collect(),
            token: Vec::new(),
            start:0,
            current:0,
            line:1,
            column:1,
            errors: Vec::new(),
        }
    }

    pub fn tokenize(&mut self)->Result<Vec<Token>, Vec<LexerErrors>>{
        while !self.is_at_end(){
            self.start=self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(
            TokenKind: EOF,
            string::new(),
            line:self.line,
            column:self.col,
        )
        )
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
            '('=>self.add_token(TokenKind:LeftParen),
            ')'=>self.add_token(TokenKind:RightParen),
            '{'=>self.add_token(TokenKind:LeftBraces),
            '}'=>self.add_token(TokenKind:RightBraces),
            ':'=>self.add_token(TokenKind:Colon),
            ';'=>self.add_token(TokenKind:SemiColon),
            ','=>self.add_token(TokenKind:Comma),
            '+'=>self.add_token(TokenKind:Plus),
            '*'=>self.add_token(TokenKind:Star),
            '%'=>self.add_token(TokenKind:Percent),

            //single or multi until understood
            '-'=>{
                if self.match_char('>'){
                    self.add_token(TokenKind:Arrow)
                }
                else{
                    self.add_token(TokenKind:Minus)
                }
            }
            '/'=>{
                if self.match_char('/'){
                    while self.peek()!='\n'&& !self.is_at_end(){
                        self.advance();
                    }
                }
                else{
                    self.add_token(TokenKind:Slash)
                }
            }
            '='=>{
                if self.match_char('='){
                    self.add_token(TokenKind:EqualEqual)
                }
                else{
                    self.add_token(TokenKind:Equal)
                }
            }
            '!'=>{
                if self.match_char('='){
                    self.add_token(TokenKind:BangEqual)
                }
                else{
                    self.add_token(TokenKind:Bang)
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenKind::LessEqual);
                } else {
                    self.add_token(TokenKind::Less);
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenKind::GreaterEqual);
                } else {
                    self.add_token(TokenKind::Greater);
                }
            }
            '&'=>{
                if self.match_char('&'){
                    self.add_token(TokenKind:And)
                }
                else{
                    self.error("Expected && for logical AND")
                }
            }
            '|'=>{
                if self.match_char('|'){
                    self.add_token(TokenKind:Or)
                }
                else{
                    self.error("Expected || for logical Or")
                }
            }

            //whitespaces and shit
            ' '|'\r'|'\t'=>{}
            '\n'=>{
                self.line += 1;
                self.column = 1;
            }

            //Strings and stuff yk
            '"'=>self.string();

            // numbers and identifiers
            _ => {
                if c.is_ascii_digit(){
                    self.number();
                }
                else if c.is_alphabetic() || c == '_'(
                    self.identifiers;
                )
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
            if(self.peek()=='\n'){
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
            })
            return;
        }
        self.advance(); //this should eat the final quote
        let value = : String = self.source[self.start+1, self.current-1]
        .iter()
        .collect();
        self.add_token(TokenKind::StringLiteral(value));
    }
    fn number(&mut self){
        while self.peek().is_ascii_digit(){
            self.advance();
        }
        if self.peek()=='.' && self.peek_next.is_ascii_digit(){
            self.advance()//DECIMAL point is consumed
            while(self.peek().is_ascii_digit(){
                self.advance();
            }
            let value:String = self.source[self.start, self.current].iter().collect();
            let float_val:f64 = value.parse().unwrap();
            self.add_token(TokenKind::Float(float_val);
        }
        else{
            while(self.peek().is_ascii_digit){
                self.advance();
            }
            let value:String = self.source[self.start,self.current].iter().collect();
            let int_val:i64 = value.parse().unwrap();
            self.add_token(TokenKind::Integer(int_val);
        }
    }
}
