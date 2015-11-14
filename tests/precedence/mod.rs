use jsrs_common::ast::BinOp::*;

#[test]
fn star() {
    assert!(Star.precedence() == Slash.precedence());
    assert!(Star.precedence() > Plus.precedence());
    assert!(Star.precedence() > Minus.precedence());
    assert!(Star.precedence() > And.precedence());
    assert!(Star.precedence() > Or.precedence());
}

#[test]
fn slash() {
    assert!(Slash.precedence() == Star.precedence());
    assert!(Slash.precedence() > Plus.precedence());
    assert!(Slash.precedence() > Minus.precedence());
    assert!(Slash.precedence() > And.precedence());
    assert!(Slash.precedence() > Or.precedence());
}

#[test]
fn plus() {
    assert!(Plus.precedence() < Star.precedence());
    assert!(Plus.precedence() < Slash.precedence());
    assert!(Plus.precedence() == Minus.precedence());
    assert!(Plus.precedence() > And.precedence());
    assert!(Plus.precedence() > Or.precedence());
}

#[test]
fn minus() {
    assert!(Minus.precedence() < Star.precedence());
    assert!(Minus.precedence() < Slash.precedence());
    assert!(Minus.precedence() == Plus.precedence());
    assert!(Minus.precedence() > And.precedence());
    assert!(Minus.precedence() > Or.precedence());
}

#[test]
fn and() {
    assert!(And.precedence() < Star.precedence());
    assert!(And.precedence() < Slash.precedence());
    assert!(And.precedence() < Plus.precedence());
    assert!(And.precedence() < Minus.precedence());
    assert!(And.precedence() > Or.precedence());
}

#[test]
fn or() {
    assert!(Or.precedence() < Star.precedence());
    assert!(Or.precedence() < Slash.precedence());
    assert!(Or.precedence() < Plus.precedence());
    assert!(Or.precedence() < Minus.precedence());
    assert!(Or.precedence() < And.precedence());
}
