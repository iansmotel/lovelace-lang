mod token;

use crate::runtime::errors;
// fn find_token(c: char) -> Token {
//     new(TokenType::LeftParenthesis, "(".to_string(), None, 1)
// }


pub struct Lexer {
    source: Vec<char>,
    tokens: Vec<token::Token>,

    had_error: bool,
    all_errors: String,
    
    start: usize,
    current: usize,
    line: usize,

}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Lexer {
            source: source.chars().collect(),
            tokens: Vec::new(),

            had_error: false,
            all_errors: String::new(),
            
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            ' ' => return,
            '\r' => return,
            '\t' => return,
            '\n' => self.line += 1,
            '(' => self.add_token(token::TokenType::LeftParenthesis, None),
            ')' => self.add_token(token::TokenType::RightParenthesis, None),
            ',' => self.add_token(token::TokenType::Comma, None),
            '+' => self.add_token(token::TokenType::Plus, None),
            '*' => self.add_token(token::TokenType::Star, None),
            '%' => self.add_token(token::TokenType::Modulo, None),
            '?' => self.add_token(token::TokenType::Question, None),
            '=' => self.add_token(token::TokenType::Equal, None),
            '-' => if self.next_is('>') {
                self.current += 1;
                self.add_token(token::TokenType::Arrow, None);
            } else {
                self.add_token(token::TokenType::Minus, None);
            },
            ':' => if self.next_is('=') {
                self.current += 1;
                self.add_token(token::TokenType::Walrus, None);
            } else {
                self.add_token(token::TokenType::Colon, None);
            },
            '!' => if self.next_is('=') {
                self.current += 1;
                self.add_token(token::TokenType::NotEqual, None);
            } else {
                self.all_errors += &errors::error(self.line, format!("Unexpected character \"!\", Did you mean \"!=\" or \"not\"?")); 
                self.had_error = true;
            },
            '<' => if self.next_is('=') {
                self.current += 1;
                self.add_token(token::TokenType::LessThanEqual, None);
            } else {
                self.add_token(token::TokenType::LessThan, None);
            },
            '>' => if self.next_is('=') {
                self.current += 1;
                self.add_token(token::TokenType::GreaterThanEqual, None);
            } else {
                self.add_token(token::TokenType::GreaterThan, None);
            },
            '/' => if self.next_is('/') {
                self.current += 1;
                while self.peek() != '\n' && self.current < self.source.len() {
                    self.advance();
                }
            } else {
                self.add_token(token::TokenType::Slash, None)
            }
            '"' => {
                while self.peek() != '"' && self.current < self.source.len() {
                    if self.peek() == '\n' { self.line += 1; }
                    self.advance();
                }
                if self.current == self.source.len() {
                    self.all_errors += &errors::error(self.line, format!("Undetermined string.")); 
                    self.had_error = true;
                }
                self.advance();
                self.add_token(token::TokenType::StringType, Some(token::TokenLiteral::StringLiteral(self.source[self.start+1..self.current-1].iter().collect())));
            },
            _ => { 
                if c.is_digit(10) {
                    while self.peek().is_digit(10) {
                        self.advance();
                    }
                    if self.peek() == '.' && self.peek_next().is_digit(10) {
                        self.advance();
                        while self.peek().is_digit(10) {
                            self.advance();
                        }    
                    }
                    self.add_token(token::TokenType::NumberType, Some(token::TokenLiteral::NumberLiteral(self.source[self.start..self.current].iter().collect::<String>().parse::<f32>().unwrap())));
                } else if c.is_alphabetic() {
                    while self.peek().is_alphanumeric() || self.peek() == '_' {
                        self.advance();
                    }
                    self.add_token(self.reserved_or_identifier(), None);
                } else {
                    self.all_errors += &errors::error(self.line, format!("Unexpected character \"{}\".", c)); 
                    self.had_error = true;
                }
            },
        }
    }

    fn peek(&self) -> char { // one character lookahead
        if self.current == self.source.len() {return '\0';}
        return self.source[self.current];
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {return '\0';}
        return self.source[self.current + 1];
    }

    fn next_is(&self, expected: char) -> bool {
        if self.current >= self.source.len() {
            return false;
        }
        if self.source[self.current] != expected {
            return false;
        }
        true
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<token::Token>, String> {
        while self.current < self.source.len() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(token::Token::new(token::TokenType::EndOfFile, "".to_string(), None, self.line));
        if self.had_error {
            return Err(self.all_errors.clone());
        }
        Ok(self.tokens.clone())
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn add_token(&mut self, token_type: token::TokenType, token_literal: Option<token::TokenLiteral>) {
        let token_lexeme: String = self.source[self.start..self.current].iter().collect();
        self.tokens.push(token::Token::new(token_type, token_lexeme, token_literal, self.line));
    }

    fn reserved_or_identifier(&self) -> token::TokenType {
        let word = self.source[self.start..self.current].iter().collect::<String>();
        if word == format!("def") {
            return token::TokenType::Def;
        } else if word == format!("nil") {
            return token::TokenType::Nil;
        } else if word == format!("false") {
            return token::TokenType::False;
        } else if word == format!("true") {
            return token::TokenType::True;
        } else if word == format!("not") {
            return token::TokenType::Not;
        } else if word == format!("or") {
            return token::TokenType::Or;
        } else if word == format!("and") {
            return token::TokenType::And;
        } else {
            return token::TokenType::Identifier;
        }
        
    }

}

