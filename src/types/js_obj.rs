use std::collections::hash_map::HashMap;
use std::collections::hash_set::HashSet;
use std::fmt::{Display, Formatter, Error};
use std::string::String;
use std::vec::Vec;

use super::allocator::Allocator;
use super::binding::UniqueBinding;
use super::js_var::{JsVar, JsKey, JsType, JsPtrEnum};

#[derive(Clone, Debug)]
pub struct JsObjStruct {
    pub proto: JsProto,
    pub name: String,
    pub dict: HashMap<JsKey, JsVar>,
}

impl JsObjStruct {
    pub fn new<T: Allocator>(proto: JsProto, name: &str,
                             kv_tuples: Vec<(JsKey, JsVar, Option<JsPtrEnum>)>,
                             allocator: &mut T) -> JsObjStruct {
        JsObjStruct {
            proto: None,
            name: String::from(name),
            dict: kv_tuples.into_iter().map(|(k, v, ptr)| {
                if let Some(ptr) = ptr {
                    allocator.alloc(v.unique.clone(), ptr).expect("Unable to allocate!"); // TODO better error handling
                }
                (k, v)
            }).collect()
        }
    }

    pub fn add_key<T: Allocator>(&mut self, k: JsKey, v: JsVar, ptr: Option<JsPtrEnum>, allocator: &mut T) {
        // If the key already exists, potentially condemn its pointer, which is being overwritten.
        if let Some(var) = self.dict.get(&k) {
            match var.t {
                JsType::JsPtr(_) => allocator.condemn(var.unique.clone()).expect("Unable to whiten!"),
                _ => {}
            }
        }
        // Then, allocate the new pointer if necessary...
        if let Some(ptr) = ptr {
            allocator.alloc(v.unique.clone(), ptr).expect("Unable to allocate!"); // TODO better error handling
        }
        // ...and insert the key & value into the dictionary blindly.
        self.dict.insert(k, v);
    }

    pub fn get_children(&self) -> HashSet<UniqueBinding> {
        let mut bindings = HashSet::new();
        for v in self.dict.values() {
            match v.t {
                JsType::JsPtr(_) => { bindings.insert(v.unique.clone()); },
                _ => (),
            }
        }
        bindings
    }
}

impl Display for JsObjStruct {
    fn fmt(&self, mut fmt: &mut Formatter) -> Result<(), Error> {
        try!(write!(fmt, "{{ "));

        for (i, (ref key, ref val)) in self.dict.iter().enumerate() {
            if i != 0 {
                try!(write!(fmt, ", "));
            }

            try!(write!(fmt, "{}: {}", key, val));
        }

        write!(fmt, " }}")
    }
}

pub type JsProto = Option<Box<JsObjStruct>>;

// TODO nice JS object creation macro
//macro_rules! js_obj {
//    ( $kt:ty : $ke:expr => $vt:ty : $ve:expr ),* {
//        {
//
//        }
//    };
//}


#[cfg(test)]
mod tests {
    // TODO tests for objs
}
