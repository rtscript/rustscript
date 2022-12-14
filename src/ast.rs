use crate::object::Object;
use std::fmt;

#[derive(Debug)]
pub struct AstToken {
    pub ast_type: AstType,
    pub literal: Option<Object>,
    lexeme: String,
}

impl AstToken {
    pub fn new(ast_type: AstType, lexeme: String, literal: Option<Object>) -> AstToken {
        AstToken {
            ast_type,
            lexeme,
            literal,
        }
    }
    pub fn ast_type(&self) -> AstType {
        self.ast_type.clone()
    }
    pub fn ast_lexeme(&self) -> &str {
        &self.lexeme
    }
}

impl fmt::Display for AstToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} {}",
            self.ast_type,
            if let Some(literal) = &self.literal {
                literal.to_string()
            } else {
                "None".to_string()
            }
        )
    }
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum AstType {
    Set,
    Break,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,
    Bang,
    BangEqual,
    Assign, // ('=')
    Equals, // ('==')
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier,
    String,
    Number,
    And,
    Class,
    Else,
    False,
    Fn,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Let,
    While,
    Eof,
    Main,
    NumberType(String),
    StringType(String),
    UnKnown,
}