use std::cell::RefCell;
use std::fmt::{self, Formatter, Debug};
use std::rc::Rc;

use backend::Backend;
use super::js_var::{JsPtrEnum, JsVar};

#[derive(Clone)]
pub struct NativeVar {
    pub var: JsVar,
    pub ptr: Option<Box<JsPtrEnum>>,
    getter: fn(Rc<RefCell<Backend>>, Option<JsPtrEnum>) -> (JsVar, Option<JsPtrEnum>),
    setter: fn(Rc<RefCell<Backend>>, Option<JsPtrEnum>, JsVar,
               Option<JsPtrEnum>) -> (JsVar, Option<JsPtrEnum>),
}

impl NativeVar {
    pub fn new(var: JsVar, ptr: Option<JsPtrEnum>,
               getter: fn(Rc<RefCell<Backend>>, Option<JsPtrEnum>) -> (JsVar, Option<JsPtrEnum>),
               setter: fn(Rc<RefCell<Backend>>, Option<JsPtrEnum>, JsVar, Option<JsPtrEnum>) -> (JsVar, Option<JsPtrEnum>))
               -> NativeVar {
        NativeVar { var: var, ptr: ptr.map(Box::new), getter: getter, setter: setter }
    }

    pub fn get(&self, backend: Rc<RefCell<Backend>>, this: Option<JsPtrEnum>) -> (JsVar, Option<JsPtrEnum>) {
        let get = self.getter;

        get(backend, this)
    }

    pub fn set(&mut self, backend: Rc<RefCell<Backend>>, this: Option<JsPtrEnum>, var: JsVar,
               ptr: Option<JsPtrEnum>) {
        let set = self.setter;
        let (var, ptr) = set(backend, this, var, ptr);

        self.var = var;
        self.ptr = ptr.map(Box::new);
    }
}

impl Debug for NativeVar {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "[native_code]")
    }
}
