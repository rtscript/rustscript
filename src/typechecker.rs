use crate::{token_type::TokenType, token::Token};
use crate::error::*;
use crate::types::*;

#[derive(Debug)]
pub struct TypeChecker {
    type_of: String,
}

impl TypeChecker {
    pub fn new() -> TypeChecker {
        TypeChecker {
            type_of: "".to_string(),
        }
    }

    pub fn expect(&self, actual_type: Types, expected_type: Types, value: TokenType, exp: Vec<TokenType>) -> Types {
        if actual_type.equals(expected_type) != true {
            println!("Types failed to match in expect for binary operation"); 
        }
        actual_type
    }

    fn get_operand_types(&self, binary_type: TokenType) -> Vec<Types> {
        let str_type = Types {
            name: "str".to_owned(),
        };
        let num_type = Types {
            name: "num".to_owned(),
        };
        let unknown_type = Types {
            name: "unkown".to_owned(),
        };
        match binary_type {
            TokenType::Plus => [str_type, num_type].to_vec(),
            TokenType::Minus => [num_type].to_vec(),
            TokenType::Slash => [num_type].to_vec(),
            TokenType::Star => [num_type].to_vec(),
            _ => [unknown_type].to_vec(),
        }
    }

    fn expect_operator_type(&self, operand_type: &Types, allowed_types: &Vec<Types>, exp: &Vec<TokenType>) {
        //plus allowed_types = num + str
        if allowed_types.into_iter().any(|t| t.equals(operand_type.clone())) != true {
            println!("String types cannot perform / * - operations");
        }
    }

    fn binary(&self, exp: &Vec<TokenType>) -> Types {
        let mut t1 = self.tc([exp[0]].to_vec());
        let mut t2 = self.tc([exp[2]].to_vec());

        let allow_types = self.get_operand_types(exp[1]);

        self.expect_operator_type(&t1, &allow_types, &exp);
        self.expect_operator_type(&t2, &allow_types, &exp);
        
        self.expect(t2, t1, exp[2], exp.to_vec())
    }

    pub fn tc(&self, exp: Vec<TokenType>) -> Types {
        if exp.len() > 1 {
            if matches!(exp[1], TokenType::Plus | TokenType::Minus | TokenType::Star | TokenType::Slash) {
                return self.binary(&exp)
            }
        }
        match exp[0] {
            TokenType::String => Types {
                name: "str".to_owned(),
            },
            TokenType::Number => Types {
                name: "num".to_owned(),
            },
            _ => Types {
                name: "unknown".to_owned(),
            },
        } 
    }

    pub fn exec(&self, exp: Vec<TokenType>) -> Types {
        self.tc(exp)
    }

    pub fn test(&self, exp: Vec<TokenType>, expected: Types) -> bool {
        if self.exec(exp).equals(expected) {
            return true;
        } else {
            println!("didn't match");
            false
        }
    }
}
