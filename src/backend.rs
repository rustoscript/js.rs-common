use std::cell::RefCell;
use std::rc::Rc;

use alloc_box::AllocBox;
use gc_error::GcError;
use types::binding::Binding;
use types::js_var::{JsPtrEnum, JsVar};

pub trait Backend {
    fn alloc(&mut self, var: JsVar, ptr: Option<JsPtrEnum>) -> Result<Binding, GcError>;
    fn load(&mut self, bnd: &Binding) -> Result<(JsVar, Option<JsPtrEnum>), GcError>;
    fn store(&mut self, var: JsVar, ptr: Option<JsPtrEnum>) -> Result<(), GcError>;
    fn get_alloc_box(&self) -> Rc<RefCell<AllocBox>>;
}
