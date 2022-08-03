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
    pub name: String,
    pub params: Vec<Params>,
    pub returns: Vec<Returns>,
}

#[derive(Debug)]
pub struct Variables {
    pub var_name: String,
    pub var_type: String,
}

#[derive(Debug)]
pub struct Context {
    pub functions: Vec<Functions>,
    pub variables: Vec<Variables>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            functions: Vec::new(),
            variables: Vec::new(),
        }
    }
    // pub fn get_name(&self) -> String {
    //     self.variables[0].var_name
    // }
    pub fn variable_context(&mut self, vname: String, vtype: String) -> Result<(), Problem> {
        let mut variable = Variables {
            var_name: vname,
            var_type: vtype,
        };

        self.variables.push(variable);

        Ok(())
    }
    
    pub fn function_context(&mut self, name: String, params: String, returns: String,) -> Result<(), Problem> {

        let mut function = Functions {
            name: name,
            params: Vec::new(),
            returns: Vec::new(),
        };

        let function_param = Params {
            param_name: params,
        };

        let return_param = Returns {
            return_type: returns,
        };

        function.params.push(function_param);
        function.returns.push(return_param);

        self.functions.push(function);

        println!("{:?}", self.functions);
        
        Ok(())
    } 
}