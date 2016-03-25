use std::fmt::{Display, Formatter, Error};

use ast::Stmt;

// For storing functions.
// In the future, this will have to store some sense of local variable scope,
// to deal with closures.
#[derive(Clone, Debug)]
pub struct JsFnStruct {
    pub name: Option<String>,
    pub params: Vec<String>,
    pub stmt: Stmt,
}

impl JsFnStruct {
    pub fn new(name: &Option<String>, params: &Vec<String>, block: &Stmt) -> JsFnStruct {
        JsFnStruct {
            name: name.clone(),
            params: params.clone(),
            stmt: block.clone(),
        }
    }
}

impl Display for JsFnStruct {
    fn fmt(&self, mut fmt: &mut Formatter) -> Result<(), Error> {
        try!(write!(fmt, "function {}(", self.name.clone().unwrap_or(String::new())));

        for (i, param) in self.params.iter().enumerate() {
            if i != 0 {
                try!(write!(fmt, ", "));
            }

            try!(write!(fmt, "{}", param));
        }

        try!(write!(fmt, ") {{\n"));
        try!(self.stmt.fmt_helper(&mut fmt, 2));
        write!(fmt, "\n}}")
    }
}

#[cfg(test)]
mod tests {
    // TODO tests for fn objs
}
