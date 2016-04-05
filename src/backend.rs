use gc_error::GcError;
use types::binding::Binding;
use types::js_var::{JsPtrEnum, JsVar};

pub trait Backend {
    fn alloc(&mut self, var: JsVar, ptr: Option<JsPtrEnum>) -> Result<Binding, GcError>;
    fn load(&mut self, bnd: &Binding) -> Result<(JsVar, Option<JsPtrEnum>), GcError>;
    fn store(&mut self, var: JsVar, ptr: Option<JsPtrEnum>) -> Result<(), GcError>;
}
