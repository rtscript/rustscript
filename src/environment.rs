use std::collections::HashMap;

use crate::ast::AstType;
use crate::{ types::Types };
use crate::error::*;

#[derive(Debug)]
pub struct TypeEnvironment {
    pub record: HashMap<AstType, Types>,
    pub parent: HashMap<AstType, Types>,
}

impl TypeEnvironment {
    pub fn new() -> TypeEnvironment {
        TypeEnvironment {
            record: HashMap::new(),
            parent: HashMap::new(),
        }
    }

    pub fn define(&mut self, vname: AstType, vtype: Types) {
        self.record.insert(vname, vtype);
    }

    pub fn lookup(&self, vname: AstType) -> Result<&Types, Problem> {
        if self.record.contains_key(&vname) {
            Ok(self.record.get(&vname).expect("Couln't get type from name"))
        } else {
            Err(Problem::fail())
        }
    }
}