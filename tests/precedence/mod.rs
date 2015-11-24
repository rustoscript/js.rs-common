use jsrs_common::ast::BinOp::*;

#[test]
fn star() {
    assert!(Star.precedence() == Slash.precedence());
    assert!(Star.precedence() > Plus.precedence());
    assert!(Star.precedence() > Minus.precedence());
    assert!(Star.precedence() > Eql.precedence());
    assert!(Star.precedence() > Neq.precedence());
    assert!(Star.precedence() > Gt.precedence());
    assert!(Star.precedence() > Ge.precedence());
    assert!(Star.precedence() > Lt.precedence());
    assert!(Star.precedence() > Le.precedence());
    assert!(Star.precedence() > And.precedence());
    assert!(Star.precedence() > Or.precedence());
}

#[test]
fn slash() {
    assert!(Slash.precedence() == Star.precedence());
    assert!(Slash.precedence() > Plus.precedence());
    assert!(Slash.precedence() > Minus.precedence());
    assert!(Slash.precedence() > Eql.precedence());
    assert!(Slash.precedence() > Neq.precedence());
    assert!(Slash.precedence() > Gt.precedence());
    assert!(Slash.precedence() > Ge.precedence());
    assert!(Slash.precedence() > Lt.precedence());
    assert!(Slash.precedence() > Le.precedence());
    assert!(Slash.precedence() > And.precedence());
    assert!(Slash.precedence() > Or.precedence());
}

#[test]
fn plus() {
    assert!(Plus.precedence() < Star.precedence());
    assert!(Plus.precedence() < Slash.precedence());
    assert!(Plus.precedence() == Minus.precedence());
    assert!(Plus.precedence() > Eql.precedence());
    assert!(Plus.precedence() > Neq.precedence());
    assert!(Plus.precedence() > Gt.precedence());
    assert!(Plus.precedence() > Ge.precedence());
    assert!(Plus.precedence() > Lt.precedence());
    assert!(Plus.precedence() > Le.precedence());
    assert!(Plus.precedence() > And.precedence());
    assert!(Plus.precedence() > Or.precedence());
}

#[test]
fn minus() {
    assert!(Minus.precedence() < Star.precedence());
    assert!(Minus.precedence() < Slash.precedence());
    assert!(Minus.precedence() == Plus.precedence());
    assert!(Minus.precedence() > Eql.precedence());
    assert!(Minus.precedence() > Neq.precedence());
    assert!(Minus.precedence() > Gt.precedence());
    assert!(Minus.precedence() > Ge.precedence());
    assert!(Minus.precedence() > Lt.precedence());
    assert!(Minus.precedence() > Le.precedence());
    assert!(Minus.precedence() > And.precedence());
    assert!(Minus.precedence() > Or.precedence());
}

#[test]
fn eql() {
    assert!(Eql.precedence() < Star.precedence());
    assert!(Eql.precedence() < Slash.precedence());
    assert!(Eql.precedence() < Plus.precedence());
    assert!(Eql.precedence() < Minus.precedence());
    assert!(Eql.precedence() == Neq.precedence());
    assert!(Eql.precedence() == Gt.precedence());
    assert!(Eql.precedence() == Ge.precedence());
    assert!(Eql.precedence() == Lt.precedence());
    assert!(Eql.precedence() == Le.precedence());
    assert!(Eql.precedence() > And.precedence());
    assert!(Eql.precedence() > Or.precedence());
}

#[test]
fn neq() {
    assert!(Neq.precedence() < Star.precedence());
    assert!(Neq.precedence() < Slash.precedence());
    assert!(Neq.precedence() < Plus.precedence());
    assert!(Neq.precedence() < Minus.precedence());
    assert!(Neq.precedence() == Eql.precedence());
    assert!(Neq.precedence() == Gt.precedence());
    assert!(Neq.precedence() == Ge.precedence());
    assert!(Neq.precedence() == Lt.precedence());
    assert!(Neq.precedence() == Le.precedence());
    assert!(Neq.precedence() > And.precedence());
    assert!(Neq.precedence() > Or.precedence());
}

#[test]
fn gt() {
    assert!(Gt.precedence() < Star.precedence());
    assert!(Gt.precedence() < Slash.precedence());
    assert!(Gt.precedence() < Plus.precedence());
    assert!(Gt.precedence() < Minus.precedence());
    assert!(Gt.precedence() == Eql.precedence());
    assert!(Gt.precedence() == Neq.precedence());
    assert!(Gt.precedence() == Ge.precedence());
    assert!(Gt.precedence() == Lt.precedence());
    assert!(Gt.precedence() == Le.precedence());
    assert!(Gt.precedence() > And.precedence());
    assert!(Gt.precedence() > Or.precedence());
}

#[test]
fn ge() {
    assert!(Ge.precedence() < Star.precedence());
    assert!(Ge.precedence() < Slash.precedence());
    assert!(Ge.precedence() < Plus.precedence());
    assert!(Ge.precedence() < Minus.precedence());
    assert!(Ge.precedence() == Eql.precedence());
    assert!(Ge.precedence() == Neq.precedence());
    assert!(Ge.precedence() == Gt.precedence());
    assert!(Ge.precedence() == Lt.precedence());
    assert!(Ge.precedence() == Le.precedence());
    assert!(Ge.precedence() > And.precedence());
    assert!(Ge.precedence() > Or.precedence());
}

#[test]
fn lt() {
    assert!(Lt.precedence() < Star.precedence());
    assert!(Lt.precedence() < Slash.precedence());
    assert!(Lt.precedence() < Plus.precedence());
    assert!(Lt.precedence() < Minus.precedence());
    assert!(Lt.precedence() == Eql.precedence());
    assert!(Lt.precedence() == Neq.precedence());
    assert!(Lt.precedence() == Gt.precedence());
    assert!(Lt.precedence() == Ge.precedence());
    assert!(Lt.precedence() == Le.precedence());
    assert!(Lt.precedence() > And.precedence());
    assert!(Lt.precedence() > Or.precedence());
}

#[test]
fn le() {
    assert!(Le.precedence() < Star.precedence());
    assert!(Le.precedence() < Slash.precedence());
    assert!(Le.precedence() < Plus.precedence());
    assert!(Le.precedence() < Minus.precedence());
    assert!(Le.precedence() == Eql.precedence());
    assert!(Le.precedence() == Neq.precedence());
    assert!(Le.precedence() == Gt.precedence());
    assert!(Le.precedence() == Ge.precedence());
    assert!(Le.precedence() == Lt.precedence());
    assert!(Le.precedence() > And.precedence());
    assert!(Le.precedence() > Or.precedence());
}

#[test]
fn and() {
    assert!(And.precedence() < Star.precedence());
    assert!(And.precedence() < Slash.precedence());
    assert!(And.precedence() < Plus.precedence());
    assert!(And.precedence() < Minus.precedence());
    assert!(And.precedence() < Eql.precedence());
    assert!(And.precedence() < Neq.precedence());
    assert!(And.precedence() < Gt.precedence());
    assert!(And.precedence() < Ge.precedence());
    assert!(And.precedence() < Lt.precedence());
    assert!(And.precedence() < Le.precedence());
    assert!(And.precedence() > Or.precedence());
}

#[test]
fn or() {
    assert!(Or.precedence() < Star.precedence());
    assert!(Or.precedence() < Slash.precedence());
    assert!(Or.precedence() < Plus.precedence());
    assert!(Or.precedence() < Minus.precedence());
    assert!(Or.precedence() < Eql.precedence());
    assert!(Or.precedence() < Neq.precedence());
    assert!(Or.precedence() < Gt.precedence());
    assert!(Or.precedence() < Ge.precedence());
    assert!(Or.precedence() < Lt.precedence());
    assert!(Or.precedence() < Le.precedence());
    assert!(Or.precedence() < And.precedence());
}
