use std::fmt::{self, Formatter, Debug};

use backend::Backend;
use super::js_var::{JsPtrEnum, JsVar};

#[derive(Clone)]
pub struct NativeFn(fn(Box<Backend>, Vec<(JsVar, Option<JsPtrEnum>)>) -> JsVar);

impl Debug for NativeFn {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "[native_code]")
    }
}
