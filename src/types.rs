use std::fmt;

use crate::ast::{AstType};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum RustScriptType {
    Number,
    String,
    UnKnown,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Types {
    pub name: RustScriptType,
}

impl fmt::Display for Types {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.name)
    }
}

impl Types {
    pub fn new(types: RustScriptType) -> Types{
        Types { name: types }
    }

    pub fn self_nb(self) -> Types {
        self
    }

    fn get_name(&self) -> RustScriptType {
        self.name.clone()
    }

    fn to_string(&self) {
        self.get_name();
    }
    
    pub fn equals(&self, other: Types) -> bool {
        // println!("Type Equality Check: {} {}", &self.get_name(), other.name);
        self.name == other.name
    }
    // meant to convert type annotation 'number'  in code for his langauge to Type.number  
    pub fn from_string(type_str: RustScriptType) -> Types {
        match type_str {
            RustScriptType::Number => Types { name: RustScriptType::Number },
            RustScriptType::String => Types { name: RustScriptType::String },
            RustScriptType::UnKnown =>  {
                println!("unknown type {:?}", &type_str);
                Types { name: RustScriptType::UnKnown }
            },
        }
    }

    pub fn type_to_token(&self) -> AstType {
        match self {
            Types { name: RustScriptType::Number } => AstType::Number,
            Types { name: RustScriptType::String }  => AstType::String,
            Types { name: RustScriptType::UnKnown }  => AstType::UnKnown,
        }
    }
}