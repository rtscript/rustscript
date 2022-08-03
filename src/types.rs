use std::fmt;

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
        println!("Here we go: {} {}", &self.get_name(), other.name);
        self.get_name() == other.name
    }
}