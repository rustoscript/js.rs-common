use std::collections::hash_map::HashMap;
use std::collections::hash_set::HashSet;
use std::fmt::{Display, Formatter, Error};
use std::string::String;
use std::vec::Vec;

use alloc_box::AllocBox;
use super::binding::UniqueBinding;
use super::js_var::{JsVar, JsKey, JsType, JsPtrEnum};

macro_rules! ptr_type_mismatch {
    ($tag:expr, $ptr:expr) => {
        panic!("Pointer type mismatch: expected {:?} but received {:?}", $tag, $ptr);
    }
}

pub type JsProto = Option<Box<JsObjStruct>>;

#[derive(Clone, Debug)]
pub struct JsObjStruct {
    pub proto: JsProto,
    pub name: String,
    pub dict: HashMap<JsKey, JsVar>,
}

impl JsObjStruct {
    #[allow(unused_variables)]
    pub fn new(proto: JsProto, name: &str, kv_tuples: Vec<(JsKey, JsVar, Option<JsPtrEnum>)>,
               allocator: &mut AllocBox) -> JsObjStruct {
        let proto_vec : Vec<_> = match proto {
            Some(ref obj) => obj.clone().dict.into_iter().map(|(key, var)| {
                let ptr = allocator.find_id(&var.unique).map(|ptr| ptr.borrow().clone());
                (key, JsVar::new(var.t), ptr)
            }).collect(),
            None => Vec::new(),
        };

        JsObjStruct {
            proto: proto,
            name: String::from(name),
            dict: proto_vec.into_iter().chain(kv_tuples.into_iter()).map(|(k, v, ptr)| {
                match v.t {
                    JsType::JsPtr(ref tag) => match ptr {
                        Some(ptr) => {
                            if tag.eq_ptr_type(&ptr) {
                                println!("{:?}", ptr);
                                allocator.alloc(v.unique.clone(), ptr)
                                    .expect("Unable to allocate!"); // TODO better error handling
                            } else {
                                ptr_type_mismatch!(tag, ptr);
                            }
                        },
                        None => ptr_type_mismatch!(tag, None::<JsPtrEnum>),
                    },
                    _ => match ptr {
                        Some(ptr) => ptr_type_mismatch!(None::<JsPtrEnum>, ptr),
                        None => {},
                    },
                }
                (k, v)
            }).collect()
        }
    }

    pub fn add_key(&mut self, obj_binding: &UniqueBinding, k: JsKey, v: JsVar,
                   _ptr: Option<JsPtrEnum>, allocator: &mut AllocBox) {
        if let Some(var) = self.dict.get(&k) {
            match var.t {
                JsType::JsPtr(_) => { allocator.condemn(var.unique.clone()).expect("Unable to whiten!") },
                _ => {}
            }
        }
        /*if let Some(ptr) = ptr {
            allocator.alloc(v.unique.clone(), ptr).expect("Unable to allocate!"); // TODO better error handling
        }*/

        match allocator.find_id(obj_binding) {
            Some(ref ptr) => match &mut *(ptr.borrow_mut()) {
                &mut JsPtrEnum::JsObj(ref mut obj) => obj.dict.insert(k.clone(), v.clone()),
                _ => panic!("Binding does not belong to an object!"),
            },
            None => panic!("No pointer with matching binding found!"),
        };

        self.dict.insert(k, v);
    }

    pub fn remove_key(&mut self, obj_binding: &UniqueBinding, k: &JsKey, allocator: &mut AllocBox)
        -> Option<(JsVar, Option<JsPtrEnum>)>{
        let var_opt = match &mut *(allocator.find_id(obj_binding).expect("No pointer with matching binding found!").borrow_mut()) {
            &mut JsPtrEnum::JsObj(ref mut obj) => obj.dict.remove(k),
            _ => panic!("Binding does not belong to an object!"),
        };

        if let Some(var) = var_opt {
            let ptr = allocator.find_id(&var.unique).map(|s| s.borrow().clone());

            if ptr.is_some() {
                allocator.condemn(var.unique.clone()).expect("Unable to whiten!");
            }

            let _ = self.dict.remove(k);
            Some((var, ptr))
        } else {
            None
        }
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
