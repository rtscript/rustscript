use crate::ast::*;
use crate::environment::TypeEnvironment;
use crate::{token_type::TokenType, token::Token};
use crate::error::*;
use crate::types::*;
use crate::environment::*;

#[derive(Debug)]
pub struct TypeChecker {
    env: TypeEnvironment,
}

impl TypeChecker {
    pub fn new() -> TypeChecker {
        TypeChecker {
            env: TypeEnvironment::new(),
        }
    }

    //Checks if both operands Types match ie str, str and num, num 
    pub fn expect(&self, actual_type: Types, expected_type: Types, value: AstType, exp: Vec<AstType>) -> Types {
        if actual_type.equals(expected_type) != true {
            println!("Types failed to match in expect for binary operation"); 
        }
        actual_type
    }

    //functions get_operand_types() and expect_operator_type() make avaiable binary operators for num and string
    // make sure strings cannot do * / - , but num can do all four.
    fn get_operand_types(&self, binary_type: AstType) -> Vec<Types> {
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
            AstType::Plus => [str_type, num_type].to_vec(),
            AstType::Minus => [num_type].to_vec(),
            AstType::Slash => [num_type].to_vec(),
            AstType::Star => [num_type].to_vec(),
            _ => [unknown_type].to_vec(),
        }
    }

    fn expect_operator_type(&self, operand_type: &Types, allowed_types: &Vec<Types>, exp: &Vec<AstType>) {
        if allowed_types.into_iter().any(|t| t.equals(operand_type.clone())) != true {
            println!("String types cannot perform / * - operations");
        }
    }

    //Get types for both operands. See binary operations available for these operands match the types
    //so only the same types can do binary operations on one another ie num + num, str + str
    fn binary(&mut self, exp: &Vec<AstType>) -> Types {
        let mut t1 = self.tc([exp[0]].to_vec());
        let mut t2 = self.tc([exp[2]].to_vec());

        let allow_types = self.get_operand_types(exp[1]);

        self.expect_operator_type(&t1, &allow_types, &exp);
        self.expect_operator_type(&t2, &allow_types, &exp);
        
        self.expect(t2, t1, exp[2], exp.to_vec())
    }

    pub fn tc(&mut self, exp: Vec<AstType>) -> Types {
        if exp.len() > 1 {
            if matches!(exp[1], AstType::Plus | AstType::Minus | AstType::Star | AstType::Slash) {
                return self.binary(&exp)
            }
            /*
            if matches!(exp[0], TokenType::Let) {
                let _tag = exp[0];
                let name = exp[1];
                let value = exp[2];

                let value_type = self.tc([value].to_vec());

                /* for explicit variable definition ie let a: num = 3;
                We need to have  this for AstType::Let and AstType::Const
                AstType::NumType for initial variable type before modifications . etc, alternatively
                AstType::Identifier with lexeme = variable_name, literal = Some(type)

                if name TokenType::Identifier has the "num" value, {
                    let (var_name, type_annotation) = TokenType::Identifier.as_string, TokenType::Identifier.liter;
                    let expected_type = from_String(type_annotation); //take the "num" string and return Type. Or just make an AstType::NumType
                    self.expect(value_type, expected_type, value, exp);
                    self.env.define(var_name, expected_type)
                }            
                */

                self.env.define(name, value_type)         
            }
             */


           /*
                println!("We couln't recognise any type");
                Types { name: "unkown".to_string(), };
            */ 
        }
        match exp[0] {
            AstType::String => Types {
                name: "str".to_owned(),
            },
            AstType::Number => Types {
                name: "num".to_owned(),
            },
            // AstType::NumberType {
            //     self.env.lookup(//var_name of this type)
            // }
            // AstType::StringType {
            //     self.env.lookup(//var_name of this token)
            // }
            _ => Types {
                name: "unknown".to_owned(),
            },
        } 
    }

    pub fn exec(&mut self, exp: Vec<AstType>) -> Types {
        self.tc(exp)
    }

    pub fn test(&mut self, exp: Vec<AstType>, expected: Types) -> bool {
        if self.exec(exp).equals(expected) {
            return true;
        } else {
            println!("Test Failed!");
            false
        }
    }
}
