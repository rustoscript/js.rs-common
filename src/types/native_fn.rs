use std::fmt::{self, Formatter, Debug};

use backend::Backend;
use super::js_var::{JsPtrEnum, JsVar};

#[derive(Clone)]
pub struct NativeFn(fn(Box<Backend>, Vec<(JsVar, Option<JsPtrEnum>)>) -> (JsVar, Option<JsPtrEnum>));

impl NativeFn {
    pub fn new(func: fn(Box<Backend>, Vec<(JsVar, Option<JsPtrEnum>)>) -> (JsVar, Option<JsPtrEnum>)) -> NativeFn 
{
      NativeFn(func)
    }

    pub fn call(&self, backend: Box<Backend>, args: Vec<(JsVar, Option<JsPtrEnum>)>) -> (JsVar, 
Option<JsPtrEnum>) {
        self.0(backend, args)
    }
}

impl Debug for NativeFn {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "[native_code]")
    }
}
