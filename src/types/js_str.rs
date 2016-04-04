use std::fmt::{Display, Formatter, Error};
use std::string::String;

// `string`
#[derive(Clone, Debug, Eq, PartialEq, Hash, HeapSizeOf)]
pub struct JsStrStruct {
    pub text: String,
}

impl JsStrStruct {
    const MAX_STR_LEN: u64 = 9007199254740991; // 2^53 - 1

    pub fn new(s: &str) -> JsStrStruct {
        assert!((s.len() as u64) < JsStrStruct::MAX_STR_LEN);
        JsStrStruct { text: s.to_string(), }
    }
}

impl Display for JsStrStruct {
    fn fmt(&self, mut fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "{}", self.text)
    }
}
