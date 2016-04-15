use std::fmt;
use std::result;
use types::js_var::{JsVar, JsPtrEnum};
use gc_error::GcError;


#[derive(Debug)]
pub enum JsError {
    ParseError(String),
    GcError(GcError),
    TypeError(String),
    ReferenceError(String),
    JsVar((JsVar, Option<JsPtrEnum>)),
    TestError(String),
    UnimplementedError(String),
}

impl JsError {
    pub fn invalid_lhs() -> JsError {
        JsError::ReferenceError(String::from("Invalid left-hand side in assignment"))
    }

    #[allow(dead_code)]
    pub fn unimplemented(typ: &str) -> JsError {
        JsError::UnimplementedError(format!("{} not implemented", typ))
    }

    /// Meta errors are problems with the interpreter -- parsing, gc, or unimplemented methods.
    pub fn is_meta_error(&self) -> bool {
        match self {
            &JsError::ParseError(_) => true,
            &JsError::GcError(_) => true,
            &JsError::TypeError(_) => false,
            &JsError::ReferenceError(_) => false,
            &JsError::JsVar(_) => false,
            &JsError::TestError(_) => false,
            &JsError::UnimplementedError(_) => true,
        }
    }
}

impl fmt::Display for JsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            JsError::ParseError(ref s) => write!(f, "ParseError: {}", s),
            JsError::GcError(ref gc) => write!(f, "GcError: {}", gc),
            JsError::TypeError(ref s) => write!(f, "TypeError: {}", s),
            JsError::ReferenceError(ref s) => write!(f, "ReferenceError: {}", s),
            JsError::JsVar(ref var_value) => write!(f, "{:?}", var_value),
            JsError::TestError(ref s) => write!(f, "TestError: {}", s),
            JsError::UnimplementedError(ref s) => write!(f, "UnimplementedError: {}", s),
        }
    }
}

impl From<GcError> for JsError {
    fn from(e: GcError)-> Self {
        JsError::GcError(e)
    }
}

pub type Result<T> = result::Result<T, JsError>;
