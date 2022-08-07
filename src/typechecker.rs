use std::collections::HashMap;
use crate::{ast::*, environment};
use crate::environment::{TypeEnvironment, Environments};
use crate::types::*;

#[derive(Debug)]
pub struct TypeChecker {
    env: Environments,
}

impl TypeChecker {
    pub fn new() -> TypeChecker {
        TypeChecker {
            env: Environments::new(),
        }
    }

    

    //functions get_operand_types() and expect_operator_type() make avaiable binary operators for num and string
    // make sure strings cannot do * / - , but num can do all four.
    fn get_operand_types(&self, binary_type: AstType) -> Vec<Types> {
        let str_type = Types::new(RustScriptType::String);
        let num_type = Types::new(RustScriptType::Number);
        let unknown_type = Types::new(RustScriptType::UnKnown);

        match binary_type {
            AstType::Plus => vec![str_type, num_type],
            AstType::Minus => vec![num_type],
            AstType::Slash => vec![num_type],
            AstType::Star => vec![num_type],
            _ => vec![unknown_type],
        }
    }

    fn expect_operator_type(&self, operand_type: &Types, allowed_types: &Vec<Types>, exp: &Vec<AstType>) {
        if allowed_types.into_iter().any(|t| t.equals(operand_type.clone())) != true {
            println!("Unexpected type: '{}' in operation '{:?}', allowed  '{:?}'",  operand_type, exp, allowed_types);
        }
    }

    fn throw(&self, actual_type: Types, expected_type: Types, value: AstType, exp: Vec<AstType>) {
        println!("Expected '{}' type for {:?} in {:?} but got {}", expected_type, value, exp, actual_type);
    }

    //Checks if both operands Types match ie str, str and num, num 
    fn expect(&self, actual_type: Types, expected_type: Types, value: AstType, exp: Vec<AstType>) -> Types {
        //value, exp for error purposes
        if actual_type.equals(expected_type.clone()) != true {
            self.throw(actual_type.clone(), expected_type, value, exp);
            // println!("Types failed to match in expect for binary operation"); 
        }
        actual_type
    }

    //Get types for both operands. See binary operations available for these operands match the types
    //so only the same types can do binary operations on one another ie num + num, str + str
    fn binary(&mut self, exp: &Vec<AstType>, env: &Option<TypeEnvironment>) -> Types {
        let t1 = self.tc(&vec![exp[0].clone()], env);
        let t2 = self.tc(&vec![exp[2].clone()], env);

        let allow_types = self.get_operand_types(exp[1].clone());

        self.expect_operator_type(&t1, &allow_types, &exp);
        self.expect_operator_type(&t2, &allow_types, &exp);
        
        self.expect(t2, t1, exp[2].clone(), exp.to_vec())
    }



    pub fn tc(&mut self, exp: &Vec<AstType>, env: &Option<TypeEnvironment>) -> Types {
        if exp.len() == 5 {
            println!("We in here");
            let mut current_type = self.tc(
                &vec![exp[0].clone(), 
                exp[1].clone(), 
                exp[2].clone()], &env
            );
            return self.tc(&vec![current_type.type_to_token(), exp[3].clone(), exp[4].clone()], &env);
        }

        if exp.len() == 3 {
            match exp[1] {
                AstType::Plus | AstType::Minus | AstType::Star | AstType::Slash => return self.binary(&exp, env),
                _ => {},
            }
        }

       
        
        match &exp[0] {
            AstType::Let => {
                let var_name = match &exp[1] {
                    AstType::NumberType(vname) => vname.to_owned(),
                    _ => "".to_owned()
                };

                let var_value = &exp[2];

                //infer value type from value
                let value_type = self.tc(&vec![var_value.to_owned()], &env);
                //Check if the annotad type :number matches type of inferred value_type, using expect   

                match env {
                    Some(env) => {
                        return Types::new(RustScriptType::UnKnown)
                    },
                    None => return self.env.global_env.define(var_name.to_owned(), value_type),
                }
            },
            AstType::String => Types {
                name: RustScriptType::String,
            },
            AstType::Number => Types {
                name: RustScriptType::Number,
            },
            AstType::NumberType(var_name) => {
                match self.env.global_env.lookup(var_name.clone()) {
                    Ok(var_type) => var_type.to_owned(),
                    Err(err) =>{
                    println!("Undefined variable {} with error {:?}", var_name, err);
                        Types { name: RustScriptType::UnKnown }
                    },
                }
            },
            AstType::StringType(var_name) => {
                match self.env.global_env.lookup(var_name.clone()) {
                    Ok(var_type) => var_type.to_owned(),
                    Err(err) =>{
                    println!("Undefined variable {} with error {:?}", var_name, err);
                        Types { name: RustScriptType::UnKnown }
                    },

                }
            },
            AstType::LeftBrace => self.tc_block(exp.to_owned(), env),
            _ =>  {
                println!("Unknown type for expression: {:?}", exp);
                return Types::new(RustScriptType::UnKnown);
            },
        } 
    }

    pub fn exec(&mut self, exp: Vec<AstType>) -> Types {
        self.tc(&exp, &None)
    }

    pub fn test(&mut self, exp: Vec<AstType>, expected: Types) -> bool {
        let actual = self.exec(exp.clone());
        if actual.equals(expected.clone()) {
            println!("Type Match!");
            true
        } else {
            println!("Expected to return '{:?}' for Expression '{:?}' but got '{:?}'", expected, &exp, actual);
            false
        }
    }

    pub fn tc_block(&mut self, exp: Vec<AstType>, env: &Option<TypeEnvironment>) -> Types {

        // we need statements of {} and statements/expressions ending in ;
        let mut statements: Vec<Vec<AstType>> = Vec::new();
        let mut temp: Vec<AstType> = Vec::new();
        
        for operands in exp[1..].iter() {
            //if we get {, keep push untill we get }
            if operands == &AstType::LeftBrace {
                while operands != &AstType::RightBrace {
                    temp.push(operands.clone());
                }
                statements.push(temp.clone());
            }

            //Push statements/expressions ending in semi-colon
            if operands != &AstType::SemiColon {
                temp.push(operands.to_owned());
            } else {
                statements.push(temp.clone());
                temp = Vec::new();
            }
            
        }

        let mut result = Types::new(RustScriptType::UnKnown);

        for stmt in statements.iter() {
            result = self.tc(stmt, &None);
            // println!("{:?}", stmt);
        }

        result
    }
}
