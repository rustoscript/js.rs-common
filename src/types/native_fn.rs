use std::cell::RefCell;
use std::fmt::{self, Formatter, Debug};
use std::mem::size_of;
use std::rc::Rc;

use backend::Backend;
use heapsize::HeapSizeOf;
use super::js_var::{JsPtrEnum, JsVar};

#[derive(Clone)]
pub struct NativeFn(fn(Rc<RefCell<Backend>>,
                       Option<JsPtrEnum>,
                       Vec<(JsVar, Option<JsPtrEnum>)>)
                    -> (JsVar, Option<JsPtrEnum>));

impl NativeFn {
    pub fn new(func: fn(Rc<RefCell<Backend>>, Option<JsPtrEnum>,
               Vec<(JsVar, Option<JsPtrEnum>)>) -> (JsVar, Option<JsPtrEnum>)) -> NativeFn
{
      NativeFn(func)
    }

    pub fn call(&self, backend: Rc<RefCell<Backend>>, this: Option<JsPtrEnum>,
                args: Vec<(JsVar, Option<JsPtrEnum>)>) -> (JsVar, Option<JsPtrEnum>) {
        self.0(backend, this, args)
    }
}

impl Debug for NativeFn {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "[native_code]")
    }
}

impl HeapSizeOf for NativeFn {
    fn heap_size_of_children(&self) -> usize {
        // Probably not accurate, but good enough for now.
        size_of::<&Fn()>()
    }
}
