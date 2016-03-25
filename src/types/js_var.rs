use std::fmt::{Display, Formatter, Error};
use std::hash::{Hash, Hasher};
use std::string::String;

use super::binding::{Binding, UniqueBinding};
use super::coerce::AsString;
use super::js_fn::JsFnStruct;
use super::js_obj::JsObjStruct;
use super::js_str::JsStrStruct;
use super::native_fn::NativeFn;

#[derive(Clone, Debug)]
pub struct JsVar {
    pub unique: UniqueBinding,
    pub binding: Binding,
    pub t: JsType,
}

impl Hash for JsVar {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        self.unique.hash(state);
    }
}

impl JsVar {
    pub fn new(t: JsType) -> JsVar {
        let mut var = JsVar {
            unique: UniqueBinding::dummy(),
            binding: Binding::anon(),
            t: t,
        };
        var.unique = UniqueBinding::mangle(&var.binding);
        var
    }

    pub fn bind(binding: &str, t: JsType) -> JsVar {
        let mut var = JsVar {
            unique: UniqueBinding::mangle_str(binding),
            binding: Binding::new(binding.to_owned()),
            t: t,
        };
        var.unique = UniqueBinding::mangle(&var.binding);
        var
    }

    pub fn type_of(&self) -> String {
        self.t.type_of()
    }
}

impl Display for JsVar {
    fn fmt(&self, mut fmt: &mut Formatter) -> Result<(), Error> {
        if !self.binding.is_anon() {
            return write!(fmt, "{}", self.binding);
        }

        write!(fmt, "{}", self.t)
    }
}

impl PartialEq for JsVar {
    fn eq(&self, other: &Self) -> bool {
        self.unique == other.unique
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Eq for JsVar {}

#[derive(Clone, Debug)]
pub enum JsPtrEnum {
    JsSym(String),
    JsStr(JsStrStruct),
    JsObj(JsObjStruct),
    JsFn(JsFnStruct),
    NativeFn(NativeFn),
}

impl Display for JsPtrEnum {
    fn fmt(&self, mut fmt: &mut Formatter) -> Result<(), Error> {
        match self {
            &JsPtrEnum::JsSym(ref s) => write!(fmt, "Symbol({})", s),
            &JsPtrEnum::JsStr(ref s) => write!(fmt, "\"{}\"", s),
            &JsPtrEnum::JsObj(ref o) => write!(fmt, "{}", o),
            &JsPtrEnum::JsFn(ref f) => write!(fmt, "{}", f),
            &JsPtrEnum::NativeFn(_) => write!(fmt, "[native code]"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum JsPtrTag {
    JsSym,
    JsStr,
    JsObj,
    JsFn { name: Option<String> },
    NativeFn { name: String },
}

impl JsPtrTag {
    pub fn eq_ptr_type(&self, other: &JsPtrEnum) -> bool {
        match (self, other) {
            (&JsPtrTag::JsSym, &JsPtrEnum::JsSym(_)) |
            (&JsPtrTag::JsStr, &JsPtrEnum::JsStr(_)) |
            (&JsPtrTag::JsObj, &JsPtrEnum::JsObj(_)) |
            (&JsPtrTag::JsFn{..},  &JsPtrEnum::JsFn(_)) |
            (&JsPtrTag::NativeFn{..}, &JsPtrEnum::NativeFn(_)) => true,
            _ => false
        }
    }

    fn type_of(&self) -> String {
        let s = match *self {
            JsPtrTag::JsSym => "symbol",
            JsPtrTag::JsStr => "string",
            JsPtrTag::JsObj => "object",
            JsPtrTag::JsFn{..} | JsPtrTag::NativeFn{..} => "function",
        };

        String::from(s)
    }
}

#[derive(Clone, Debug)]
pub enum JsType {
    JsUndef,
    JsNum(f64),
    JsBool(bool),
    JsPtr(JsPtrTag),
    JsNull, // null is not a ptr since it doesn't actually require heap allocation
}

impl JsType {
    fn type_of(&self) -> String {
        let s = match *self {
            JsType::JsUndef => "undefined",
            JsType::JsNum(_) => "number",
            JsType::JsBool(_) => "boolean",
            JsType::JsNull => "object",
            JsType::JsPtr(ref t) => return t.type_of(),
        };

        String::from(s)
    }
}

impl Display for JsType {
    fn fmt(&self, mut fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "{}", self.as_string())
    }
}

impl PartialEq for JsType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&JsType::JsUndef, &JsType::JsUndef) => true,
            (&JsType::JsNum(x), &JsType::JsNum(y)) => x == y,
            (&JsType::JsBool(b1), &JsType::JsBool(b2)) => b1 == b2,
            (&JsType::JsNull, &JsType::JsNull) => true,
            (_, _) => false,
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Eq for JsType {}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum JsKey {
    JsStr(JsStrStruct),
    JsSym(String),
}

impl Display for JsKey {
    fn fmt(&self, mut fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            JsKey::JsStr(ref s) => write!(fmt, "{}", s),
            JsKey::JsSym(ref s) => write!(fmt, "Symbol({})", s),
        }
    }
}
