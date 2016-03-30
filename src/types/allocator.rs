use std::fmt::Debug;

use super::binding::UniqueBinding;
use super::js_var::JsPtrEnum;

pub trait Allocator {
    type Error: Debug;
    fn alloc(&mut self, binding: UniqueBinding, ptr: JsPtrEnum) -> Result<(), Self::Error>;
    fn condemn(&mut self, unique: UniqueBinding) -> Result<(), Self::Error>;
}
