use std::collections::HashMap;
use crate::{ types::Types};
use crate::error::*;

#[derive(Debug, Clone)]
pub struct TypeEnvironment {
    pub record: HashMap<String, Types>,
    pub parent: HashMap<String, Types>,
}

#[derive(Debug, Clone)]
pub struct Environments {
    pub global_env: TypeEnvironment,
    pub branch: Vec<TypeEnvironment>
}

impl Environments {
    pub fn new() -> Environments {
        Environments { global_env: TypeEnvironment::new(), branch: vec![TypeEnvironment::new()] }
    }
}

impl TypeEnvironment {
    pub fn new() -> TypeEnvironment {
        TypeEnvironment {
            record: HashMap::new(),
            parent: HashMap::new(),
        }
    }


    pub fn define(&mut self, vname: String, vtype: Types) -> Types {
        self.record.insert(vname, vtype.to_owned());
        vtype.to_owned()
    }

    pub fn lookup(&mut self, vname: String) -> Result<&Types, Problem> {
        self.resolve(vname)
    }

    pub fn resolve(&mut self, vname: String) -> Result<&Types, Problem> {
        if self.record.contains_key(&vname) {
            Ok(self.record.get(&vname).expect("Couln't get type from name"))
        } else {
            Err(Problem::fail())
        }
    }
}