use std::fmt::{self, Formatter, Debug};

use gc_error::GcError;
use super::binding::Binding;
use super::js_var::{JsPtrEnum, JsVar};

pub trait JsScope {
    fn alloc(&mut self, var: JsVar, ptr: Option<JsPtrEnum>) -> Result<Binding, GcError>;
    fn load(&self, bnd: &Binding) -> Result<(JsVar, Option<JsPtrEnum>), GcError>;
    fn store(&mut self, var: JsVar, ptr: Option<JsPtrEnum>) -> Result<(), GcError>;
}

#[derive(Clone)]
pub struct NativeFn(fn(Box<JsScope>, Vec<(JsVar, Option<JsPtrEnum>)>) -> JsVar);

impl Debug for NativeFn {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "[native_code]")
    }
}
