use std::cell::RefCell;
use std::fmt::{self, Formatter, Debug};
use std::rc::Rc;

use backend::Backend;
use super::js_str::JsStrStruct;
use super::js_var::{JsKey, JsPtrEnum, JsPtrTag, JsType, JsVar};

#[derive(Clone)]
pub struct NativeVar {
    pub var: JsVar,
    pub ptr: Option<Box<JsPtrEnum>>,
    name: String,
    getter: fn(Rc<RefCell<Backend>>, JsVar, Option<JsPtrEnum>, Option<JsPtrEnum>) -> (JsVar, Option<JsPtrEnum>),
    setter: fn(Rc<RefCell<Backend>>, JsVar, Option<JsPtrEnum>, Option<JsPtrEnum>, JsVar,
               Option<JsPtrEnum>) -> (JsVar, Option<JsPtrEnum>),
}

impl NativeVar {
    pub fn new(var: JsVar, ptr: Option<JsPtrEnum>, name: &str,
               getter: fn(Rc<RefCell<Backend>>, JsVar, Option<JsPtrEnum>, Option<JsPtrEnum>) -> (JsVar, Option<JsPtrEnum>),
               setter: fn(Rc<RefCell<Backend>>, JsVar, Option<JsPtrEnum>, Option<JsPtrEnum>, JsVar, Option<JsPtrEnum>) -> (JsVar, Option<JsPtrEnum>))
               -> NativeVar {
        NativeVar { var: var, ptr: ptr.map(Box::new), name: String::from(name), getter: getter, setter: setter }
    }

    pub fn get(&self, backend: Rc<RefCell<Backend>>, this: Option<JsPtrEnum>) -> (JsVar, Option<JsPtrEnum>) {
        let get = self.getter;

        get(backend, self.var.clone(), self.ptr.clone().map(|x| *x), this)
    }

    pub fn set(&mut self, state: Rc<RefCell<Backend>>, this: Option<JsPtrEnum>, var: JsVar,
               ptr: Option<JsPtrEnum>) {
        let set = self.setter;
        let (var, ptr) = set(state.clone(), self.var.clone(), self.ptr.clone().map(|x| *x), this.clone(), var, ptr);

        self.var = var;
        self.ptr = ptr.map(Box::new);

        if let Some(JsPtrEnum::JsObj(mut obj)) = this {
            let key = JsKey::JsStr(JsStrStruct::new(&self.name));
            let state_ref = state.borrow_mut();
            let alloc_box = state_ref.get_alloc_box();
            let self_var = JsVar::new(JsType::JsPtr(JsPtrTag::NativeVar { type_string: String::from("number") }));
            let self_ptr = JsPtrEnum::NativeVar(self.clone());
            obj.add_key(key, self_var, Some(self_ptr), &mut *(alloc_box.borrow_mut()));
        }
    }
}

impl Debug for NativeVar {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        self.ptr.as_ref().map(|p| write!(fmt, "{:?}", p)).unwrap_or(write!(fmt, "{:?}", self.var))
    }
}
