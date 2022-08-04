use std::fmt;

use crate::token_type::TokenType;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Types {
    pub name: String,
}

impl fmt::Display for Types {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Types {
    pub fn new(&self, tname: String) -> Self {
        Types {
            name: tname,
        }
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn to_string(&self) {
        self.get_name();
    }
    
    pub fn equals(&self, other: Types) -> bool {
        // println!("Type Equality Check: {} {}", &self.get_name(), other.name);
        self.get_name() == other.name
    }
    // 'number' to Type.number  TokenType::Number -> Types {names: "num"}
    // 5:05 episode 4: Folder 1.
    pub fn from_string(type_str: String) -> Types {
        Types { name: "num".to_string() }
    }
}