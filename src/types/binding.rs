use std::fmt;

use uuid::Uuid;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Binding(pub String);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct UniqueBinding(pub String);

impl Binding {
    pub fn new(s: String) -> Self {
        Binding(s)
    }

    pub fn mangle(b: &Self) -> Self {
        Self::new(String::from("%---") +  &b.0 +  "---%" +
                  &Uuid::new_v4().to_simple_string())
    }

    pub fn anon() -> Self {
        Self::mangle(&Self::new(">anon_js_var<".to_string()))
    }

    pub fn is_anon(&self) -> bool {
        self.0.contains(">anon_js_var<")
    }

    pub fn var_name(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Binding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl UniqueBinding {
    pub fn new(s: String) -> Self {
        UniqueBinding(s)
    }

    pub fn mangle(b: &Binding) -> Self {
        Self::new(String::from("%---") +  &b.0 +  "---%" +
                  &Uuid::new_v4().to_simple_string())
    }

    pub fn mangle_str(s: &str) -> Self {
        Self::new(String::from("%---") +  s +  "---%" +
                  &Uuid::new_v4().to_simple_string())
    }

    pub fn anon() -> Self {
        Self::mangle(&Binding::new(">anon_js_var<".to_string()))
    }

    pub fn is_anon(&self) -> bool {
        self.0.contains(">anon_js_var<")
    }

    pub fn dummy() -> Self {
        UniqueBinding("".to_owned())
    }
}

impl fmt::Display for UniqueBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let bnd = UniqueBinding::anon();
        println!("{}", bnd);
    }
}
