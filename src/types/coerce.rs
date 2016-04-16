use std::f64::NAN;

use super::js_str::JsStrStruct;
use super::js_var::{JsPtrEnum, JsPtrTag, JsType, JsVar};
use super::js_var::JsType::*;
use super::native_var::NativeVar;

pub trait AsBool {
    fn as_bool(&self) -> bool;
}

impl AsBool for JsVar {
    fn as_bool(&self) -> bool {
        match self.t {
            JsBool(b) => b,
            JsUndef=> false,
            JsNull => false,
            JsNum(n) =>
                if n == 0.0f64 || n == -0.0f64 || n.is_nan() {
                    false
                } else {
                    true
                },
            JsPtr(_) => true, // TODO - this is incorrect
            //JsString(ref s) =>
            //    if s.len() == 0 {
            //        false
            //    } else {
            //        true
            //    },
            //JsSymbol(_) => true,
            //JsObject | JsError(_) | JsFunction(_, _, _) => true,
        }
    }
}

impl AsBool for JsPtrEnum {
    fn as_bool(&self) -> bool {
        match *self {
            JsPtrEnum::JsSym(_) |
            JsPtrEnum::JsFn(_) |
            JsPtrEnum::JsObj(_) |
            JsPtrEnum::NativeFn(_) => true,
            JsPtrEnum::JsStr(ref s) => s.text.len() != 0,
            JsPtrEnum::NativeVar(ref nv) => nv.ptr.as_ref().map(|ptr| ptr.as_bool()).unwrap_or(nv.var.as_bool()),
        }
    }
}


pub trait AsNumber {
    fn as_number(&self) -> f64;
}

impl AsNumber for JsVar {
    fn as_number(&self) -> f64 {
        match self.t {
            JsBool(b) => if b { 1f64 } else { 0f64 },
            JsUndef => NAN,
            JsNull => 0f64,
            JsNum(n) => n,
            JsPtr(_) => NAN,
        }
    }
}

impl AsNumber for JsPtrEnum {
    fn as_number(&self) -> f64 {
        match self {
            // TODO: Change this to throw a TypeError
            &JsPtrEnum::JsSym(_) => panic!("Cannot convert a Symbol to a number."),
            &JsPtrEnum::JsStr(JsStrStruct { ref text }) => {
                if text.len() == 0 {
                    return 0.0
                }

                text.parse().unwrap_or(NAN)
            }
            &JsPtrEnum::NativeVar(NativeVar { ref var, ref ptr, .. }) => match (var, ptr) {
                (_, &Some(ref ptr)) => ptr.as_number(),
                (ref var, &None) => var.as_number(),
            },
            _ => NAN
        }
    }
}

pub trait AsString {
    fn as_string(&self) -> String;
}

impl AsString for JsType {
    fn as_string(&self) -> String {
        let s = match *self {
            JsBool(true) => "true",
            JsBool(false) => "false",
            JsUndef => "undefined",
            JsNull => "null",
            JsNum(n) => return format!("{}", n),

            // NOTE: These cases should never actually be used; in the case that the variable is
            // a JsPtr, the corresponding JsPtrEnum's string coercion should be used.
            JsPtr(JsPtrTag::JsSym) => "Symbol(...)",
            JsPtr(JsPtrTag::JsStr) => "\"...\"",
            JsPtr(JsPtrTag::JsObj) => "{ ... }",
            JsPtr(JsPtrTag::JsFn{..}) => "function() { ... }",
            JsPtr(JsPtrTag::NativeFn{..}) => "function() { [native code] }",
            JsPtr(JsPtrTag::NativeVar{..}) => "[native code]",
        };

        String::from(s)
    }
}

impl AsString for JsPtrEnum {
    fn as_string(&self) -> String {
        match *self {
            JsPtrEnum::JsSym(ref s) => format!("Symbol({})", s),
            JsPtrEnum::JsStr(ref s) => s.text.to_owned(),

            // TODO: Check object's `toString` method
            JsPtrEnum::JsObj(_) => String::from("[object Object]"),

            // TODO: A function's string representation is apparently the string of source code that
            // created it; the AST doesn't currently support this, so we'll need to do some
            // restructuring before we can support this.
            JsPtrEnum::JsFn(_) => String::from("[function]"),
            JsPtrEnum::NativeFn(_) => String::from("[native function]"),
            JsPtrEnum::NativeVar(NativeVar { ref var, ref ptr, ..}) => ptr.as_ref().map(|p| p.as_string()).unwrap_or(var.t.as_string()),
        }
    }
}
