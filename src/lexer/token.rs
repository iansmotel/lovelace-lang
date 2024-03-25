use std::fmt;

#[derive(Clone)]
#[derive(Debug)] //temporary
pub enum TokenType { 
    LeftParenthesis, RightParenthesis, Plus, Minus,
    Slash, Star, Modulo, Comma,

    GreaterThan, LessThan, GreaterThanEqual, LessThanEqual,
    Equal, NotEqual, Question, Colon, Walrus,

    Identifier, StringType, NumberType,

    Arrow,

    // reserved keywords
    And, Or, Not, True, False, Nil, Def,

    EndOfFile,
}

#[derive(Clone)]
#[derive(Debug)] //temporary
pub enum TokenLiteral {
    NumberLiteral (f32),
    StringLiteral (String),
    // Identifier (String),
}

#[derive(Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<TokenLiteral>,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<TokenLiteral>, line: usize) -> Token {
        Token {token_type, lexeme, literal, line}
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self {
           _ => write!(f, "[Type: {:?}, Literal: {:?}, Lexeme: \"{}\", Line: {}]", self.token_type, self.literal, self.lexeme, self.line),
       }
    }
}

