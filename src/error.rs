use crate::object::*;
use crate::token::*;
use crate::token_type::*;

pub enum Problem {
    ParseError { token: Token, message: String },
    RuntimeError { token: Token, message: String },
    Error { line: usize, message: String },
    SystemError { message: String },
    ReturnValue { value: Object },
    Break,
    Fail,
}

impl Problem {
    pub fn fail() -> Problem {
        Problem::Fail
    }

    pub fn return_value(value: Object) -> Problem {
        Problem::ReturnValue { value }
    }

    pub fn error(line: usize, message: &str) -> Problem {
        let err = Problem::Error {
            line,
            message: message.to_string(),
        };
        err.report("");
        err
    }

    pub fn parse_error(token: &Token, message: &str) -> Problem {
        let err = Problem::ParseError {
            token: token.dup(),
            message: message.to_string(),
        };
        err.report("");
        err
    }

    pub fn runtime_error(token: &Token, message: &str) -> Problem {
        let err = Problem::RuntimeError {
            token: token.dup(),
            message: message.to_string(),
        };
        err.report("");
        err
    }

    pub fn system_error(message: &str) -> Problem {
        let err = Problem::SystemError {
            message: message.to_string(),
        };
        err.report("");
        err
    }

    fn report(&self, loc: &str) {
        match self {
            Problem::ParseError { token, message } => {
                if token.is(TokenType::Eof) {
                    eprintln!("[line {}] Error at end: {}", token.line, message);
                } else {
                    eprintln!(
                        "[line {}] Error at '{}': {}",
                        token.line,
                        token.as_string(),
                        message
                    );
                }
            }
            Problem::RuntimeError { token, message } => {
                if token.is(TokenType::Eof) {
                    eprintln!("[line {}] Error at end: {}", token.line, message);
                } else {
                    eprintln!("{}\n[line {}]", message, token.line);
                }
            }
            Problem::Error { line, message } => {
                eprintln!("[line {}] Error{}: {}", line, loc, message);
            }
            Problem::SystemError { message } => {
                eprintln!("System Error: {message}");
            }
            Problem::Break | Problem::ReturnValue { .. } => {}
            Problem::Fail => {
                panic!("should not get here")
            }
        };
    }
}
