use std::collections::HashMap;

use crate::error::*;

#[derive(Debug)]
pub struct Params {
    pub param_name: String,
}
#[derive(Debug)]
pub struct Returns {
    pub return_type: String,
}

#[derive(Debug)]
pub struct Functions {
    pub params: Vec<Params>,
    pub returns: Vec<Returns>,
}

#[derive(Debug)]
pub struct Context {
    pub functions: HashMap<String, Functions>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            functions: HashMap::new()
        }
    }
    pub fn establish_context(
        &mut self,
        name: String, 
        params: String, 
        returns: String,
    ) -> Result<(), Problem> {

        let mut functions = Functions {
            params: Vec::new(),
            returns: Vec::new(),
        };

        let function_param = Params {
            param_name: params,
        };

        let return_param = Returns {
            return_type: returns,
        };

        functions.params.push(function_param);
        functions.returns.push(return_param);

        let mut context = Context {
            functions: HashMap::new()
        };

        self.functions.insert(name, functions);



        println!("{:?}", self.functions);
        
        Ok(())
    } 
}