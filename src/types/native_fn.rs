use std::cell::RefCell;
use std::fmt::{self, Formatter, Debug};
use std::rc::Rc;

use backend::Backend;
use super::js_var::{JsPtrEnum, JsVar};
use js_error;

#[derive(Clone)]
pub struct NativeFn(fn(Rc<RefCell<Backend>>,
                       Option<(JsVar, JsPtrEnum)>,
                       Vec<(JsVar, Option<JsPtrEnum>)>)
                    -> js_error::Result<(JsVar, Option<JsPtrEnum>)>);

impl NativeFn {
    pub fn new(func: fn(Rc<RefCell<Backend>>, Option<(JsVar, JsPtrEnum)>,
               Vec<(JsVar, Option<JsPtrEnum>)>) -> js_error::Result<(JsVar, Option<JsPtrEnum>)>)
               -> NativeFn {
        NativeFn(func)
    }

    pub fn call(&self, backend: Rc<RefCell<Backend>>, this: Option<(JsVar, JsPtrEnum)>,
                args: Vec<(JsVar, Option<JsPtrEnum>)>)
                -> js_error::Result<(JsVar, Option<JsPtrEnum>)> {
        self.0(backend, this, args)
    }
}

impl Debug for NativeFn {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "[native_code]")
    }
}
