use std::collections::HashMap;
use crate::{ types::Types};
use crate::error::*;

#[derive(Debug, Clone)]
pub struct TypeEnvironment {
    pub record: HashMap<String, Types>,
    pub parent: HashMap<String, Types>,
}

impl TypeEnvironment {
    pub fn new() -> TypeEnvironment {
        TypeEnvironment {
            record: HashMap::new(),
            parent: HashMap::new(),
        }
    }

    pub fn branch_env(parent: HashMap<String, Types>) -> TypeEnvironment {
        TypeEnvironment { record: HashMap::new(), parent, }
    }


    pub fn define(&mut self, vname: String, vtype: Types) -> Types {
        self.record.insert(vname, vtype.to_owned());
        vtype.to_owned()
    }

    pub fn lookup(&mut self, vname: String) -> Result<&Types, Problem> {
        self.resolve(vname)
    }

    //Problem Very nature of adding record env to scope underneath and not 2 scopes
    //down means scope 3 branches down will not have access to global scope
    //May have to copy envs .records + .parent to child environment
    //Alternatively create self-referentials structs using Box perhaps? thus 
    //parents is contains a TypeEnvironment copy of its parent, then recursively
    //go up parent environements to find variables 
    pub fn resolve(&mut self, vname: String) -> Result<&Types, Problem> {
        if self.record.contains_key(&vname) {
            Ok(self.record.get(&vname).expect("Couln't get type from records for variable"))
        } else if self.parent.is_empty() {
            println!("Variable not defined {}", &vname);
            Err(Problem::fail())
        } else if self.parent.contains_key(&vname) {
            Ok(self.parent.get(&vname).expect("Couln't get type from parent for variable"))
        } else {
            Err(Problem::fail())
        }
    }
}