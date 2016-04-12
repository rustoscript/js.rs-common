use std::cell::RefCell;
use std::rc::Rc;

use alloc_box::AllocBox;
use ast::{Exp, Stmt};
use types::js_fn::JsFnStruct;
use types::js_obj::JsObjStruct;
use types::js_str::JsStrStruct;
use types::js_var::{JsKey, JsPtrEnum, JsPtrTag, JsType, JsVar};

pub fn make_str(s: &str) -> (JsVar, JsPtrEnum) {
    let var = JsVar::new(JsType::JsPtr(JsPtrTag::JsStr));
    (var, JsPtrEnum::JsStr(JsStrStruct::new(s)))
}

pub fn make_num(i: f64) -> JsVar {
    JsVar::new(JsType::JsNum(i))
}

pub fn make_obj(kvs: Vec<(JsKey, JsVar, Option<JsPtrEnum>)>, heap: Rc<RefCell<AllocBox>>) -> (JsVar, JsPtrEnum) {
    let var = JsVar::new(JsType::JsPtr(JsPtrTag::JsObj));
    (var, JsPtrEnum::JsObj(JsObjStruct::new(None, "test", kvs, &mut *heap.borrow_mut())))
}

pub fn make_fn(name: &Option<String>, params: &Vec<String>) -> (JsVar, JsPtrEnum) {
    let var = JsVar::new(JsType::JsPtr(JsPtrTag::JsFn { name: None }));
    (var, JsPtrEnum::JsFn(JsFnStruct::new(name, params, &vec![Stmt::BareExp(Exp::Undefined)])))
}

pub fn make_alloc_box() -> Rc<RefCell<AllocBox>> {
    Rc::new(RefCell::new(AllocBox::new()))
}
