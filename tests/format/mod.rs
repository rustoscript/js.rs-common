use jsrs_common::ast::BinOp::*;
use jsrs_common::ast::Exp::*;
use jsrs_common::ast::Stmt::*;
use std::f64::NAN;

macro_rules! format_exp {
    ($e1:expr, $o:expr, $e2:expr) => { &format!("{}", exp!($e1, $o, $e2)) }
}

macro_rules! format_assign {
    ($s:expr, $e:expr) => { &format!("{}", assign!($s, $e)) }
}

macro_rules! format_bare_exp {
    ($e:expr) => { &format!("{}", BareExp($e)) }
}

macro_rules! format_decl {
    ($s:expr, $e:expr) => { &format!("{}", decl!($s, $e)) }
}

macro_rules! format_seq {
    ($s1:expr, $s2:expr) => { &format!("{}", seq!($s1, $s2)) }
}

macro_rules! pos_float {
    ($e:expr) => { Pos(Box::new(Float($e))) }
}

#[test]
fn constants() {
    // Floating point values specifically chosen so fractional part can be perfectly represented
    // by floating-point numbers.
    assert_eq!("12.25", format!("{}", Float(12.25)));
    assert_eq!("-3.5", format!("{}", Float(-3.5)));
    assert_eq!("NaN", format!("{}", Float(NAN)));
    assert_eq!("undefined", format!("{}", Undefined));
    assert_eq!("false", format!("{}", Bool(false)));
    assert_eq!("true", format!("{}", Bool(true)));
}

#[test]
fn vars() {
    assert_eq!("x", format!("{}", var!("x")));
    assert_eq!("X", format!("{}", var!("X")));
    assert_eq!("_x", format!("{}", var!("_x")));
    assert_eq!("_x2", format!("{}", var!("_x2")));
    assert_eq!("xX_", format!("{}", var!("xX_")));
    assert_eq!("X_x", format!("{}", var!("X_x")));
    assert_eq!("-x", format!("{}", neg_var!("x")));
    assert_eq!("+_2", format!("{}", pos_var!("_2")));
}

#[test]
fn single_binop_exprs() {
    assert_eq!("-14 * -num", format_exp!(Float(-14.0), Star, neg_var!("num")));
    assert_eq!("+z++ - -10", format_exp!(post_inc!(pos_var!("z")), Minus, Float(-10.0)));
    assert_eq!("12.25 + 72", format_exp!(Float(12.25), Plus, Float(72.0)));
    assert_eq!("-3 * 42.5", format_exp!(Float(-3.0), Star, Float(42.5)));
    assert_eq!("22 / x", format_exp!(Float(22.0), Slash, var!("x")));
    assert_eq!("3 - -----y", format_exp!(Float(3.0), Minus, pre_dec!(pre_dec!(neg_var!("y")))));
    assert_eq!("_L2-- == -55.5", format_exp!(post_dec!(var!("_L2")), Eql, Float(-55.5)));
    assert_eq!("+22 != Z", format_exp!(pos_float!(22.0), Neq, var!("Z")));
    assert_eq!("-y > xX", format_exp!(neg_var!("y"), Gt, var!("xX")));
    assert_eq!("39 >= -76.25", format_exp!(Float(39.0), Ge, Float(-76.25)));
    assert_eq!("+++32 < -num--", format_exp!(pre_inc!(pos_float!(32.0)), Lt, post_dec!(neg_var!("num"))));
    assert_eq!("X_x <= 54.5", format_exp!(var!("X_x"), Le, Float(54.5)));
    assert_eq!("12++ && x_92", format_exp!(post_inc!(Float(12.0)), And, var!("x_92")));
    assert_eq!("99 || false", format_exp!(Float(99.0), Or, Bool(false)));
}

#[test]
fn multi_binop_exprs_no_grouping() {
    assert_eq!("x + 18.5 - 17",
        format_exp!(exp!(var!("x"), Plus, Float(18.5)), Minus, Float(17.0)));
    assert_eq!("-10 / +num - 17",
        format_exp!(exp!(Float(-10.0), Slash, pos_var!("num")), Minus, Float(17.0)));
    assert_eq!("-10 * 18.5 + -some_num",
        format_exp!(exp!(Float(-10.0), Star, Float(18.5)), Plus, neg_var!("some_num")));
    assert_eq!("anotherNumber - _x / 17",
        format_exp!(var!("anotherNumber"), Minus, exp!(var!("_x"), Slash, Float(17.0))));
    assert_eq!("x <= 32 && y++ == 88.5",
        format_exp!(exp!(var!("x"), Le, Float(32.0)), And, exp!(post_inc!(var!("y")), Eql, Float(88.5))));
    assert_eq!("17 < +NUMBER---- * -3 + w && false || some_bool",
        format_exp!(exp!(exp!(Float(17.0), Lt, exp!(exp!(post_dec!(post_dec!(pos_var!("NUMBER"))), Star, Float(-3.0)), Plus, var!("w"))),
            And, Bool(false)), Or, var!("some_bool")));
    assert_eq!("_ - 18.5 + x2 * -3.25",
        format_exp!(exp!(var!("_"), Minus, Float(18.5)), Plus, exp!(var!("x2"), Star, Float(-3.25))));
    assert_eq!("NUMBER * 18.5 / 17 + ---N_4",
        format_exp!(exp!(exp!(var!("NUMBER"), Star, Float(18.5)), Slash, Float(17.0)), Plus, pre_dec!(neg_var!("N_4"))));
}

#[test]
fn multi_binop_exprs_with_grouping() {
    assert_eq!("_7 * (e_4_4 + OkDk)",
        format_exp!(var!("_7"), Star, exp!(var!("e_4_4"), Plus, var!("OkDk"))));
    assert_eq!("(-10 - 18.5) / 17",
        format_exp!(exp!(Float(-10.0), Minus, Float(18.5)), Slash, Float(17.0)));
    assert_eq!("-10 - (O_k + 17)",
        format_exp!(Float(-10.0), Minus, exp!(var!("O_k"), Plus, Float(17.0))));
    assert_eq!("-10 / (O_k * 17)",
        format_exp!(Float(-10.0), Slash, exp!(var!("O_k"), Star, Float(17.0))));
    assert_eq!("+(++(((((a || b) && c) == d) + e) * f))",
        &format!("{}", Pos(Box::new(pre_inc!(
            exp!(exp!(exp!(exp!(exp!(var!("a"), Or, var!("b")), And, var!("c")), Eql, var!("d")), Plus, var!("e")), Star, var!("f")))))));
    assert_eq!("(_Ok - h2o) * 17 / -3.25",
        format_exp!(exp!(var!("_Ok"), Minus, var!("h2o")), Star, exp!(Float(17.0), Slash, Float(-3.25))));
    assert_eq!("(-10 + 18.5 - 17) * -3.25",
        format_exp!(exp!(exp!(Float(-10.0), Plus, Float(18.5)), Minus, Float(17.0)), Star, Float(-3.25)));
    assert_eq!("(-10 - (18.5 - 17)) / -3.25",
        format_exp!(exp!(Float(-10.0), Minus, exp!(Float(18.5), Minus, Float(17.0))), Slash, Float(-3.25)));
}

#[test]
fn assign_stmts() {
    assert_eq!("x = NaN;\n", format_assign!("x", Float(NAN)));
    assert_eq!("someThing = 8.25 - OTHER;\n",
        format_assign!("someThing", exp!(Float(8.25), Minus, var!("OTHER"))));
    assert_eq!("thing2 = r * -51 + 3.5;\n",
        format_assign!("thing2", exp!(exp!(var!("r"), Star, Float(-51.0)), Plus, Float(3.5))));
    assert_eq!("_2 = (-42.5 + undefined) / 7.125;\n",
        format_assign!("_2", exp!(exp!(Float(-42.5), Plus, Undefined), Slash, Float(7.125))));
}

#[test]
fn bare_exp_stmts() {
    assert_eq!("-3.5;\n", format_bare_exp!(Float(-3.5)));
    assert_eq!("NaN;\n", format_bare_exp!(Float(NAN)));
    assert_eq!("x - -10;\n", format_bare_exp!(exp!(var!("x"), Minus, Float(-10.0))));
    assert_eq!("-3 * 42.5;\n", format_bare_exp!(exp!(Float(-3.0), Star, Float(42.5))));
    assert_eq!("-10 / num - 17;\n",
        format_bare_exp!(exp!(exp!(Float(-10.0), Slash, var!("num")), Minus, Float(17.0))));
    assert_eq!("NUMBER * 18.5 / 17 + N_4;\n",
        format_bare_exp!(exp!(exp!(exp!(var!("NUMBER"), Star, Float(18.5)), Slash, Float(17.0)), Plus, var!("N_4"))));
    assert_eq!("-10 / (O_k * 17);\n",
        format_bare_exp!(exp!(Float(-10.0), Slash, exp!(var!("O_k"), Star, Float(17.0)))));
    assert_eq!("(-10 - (18.5 - 17)) / -3.25;\n",
        format_bare_exp!(exp!(exp!(Float(-10.0), Minus, exp!(Float(18.5), Minus, Float(17.0))), Slash, Float(-3.25))));
}

#[test]
fn decl_stmts() {
    assert_eq!("var _Nu4 = 2.25;\n", format_decl!("_Nu4", Float(2.25)));
    assert_eq!("var nU_M = x * -72;\n", format_decl!("nU_M", exp!(var!("x"), Star, Float(-72.0))));
    assert_eq!("var NUM = -34.5 / _l4 + 8;\n",
        format_decl!("NUM", exp!(exp!(Float(-34.5), Slash, var!("_l4")), Plus, Float(8.0))));
    assert_eq!("var eleven = (Y + 3) * -11;\n",
        format_decl!("eleven", exp!(exp!(var!("Y"), Plus, Float(3.0)), Star, Float(-11.0))));
}

#[test]
fn seq_stmts() {
    assert_eq!("someThing = 8.25 - OTHER;\n-3 * 42.5;\n",
        format_seq!(assign!("someThing", exp!(Float(8.25), Minus, var!("OTHER"))),
            BareExp(exp!(Float(-3.0), Star, Float(42.5)))));
    assert_eq!("-10 / num - 17;\nvar NUM = -34.5 / _l4 + 8;\nthing2 = r * -51 + 3.5;\n", format_seq!(
        BareExp(exp!(exp!(Float(-10.0), Slash, var!("num")), Minus, Float(17.0))), seq!(
            decl!("NUM", exp!(exp!(Float(-34.5), Slash, var!("_l4")), Plus, Float(8.0))),
            assign!("thing2", exp!(exp!(var!("r"), Star, Float(-51.0)), Plus, Float(3.5))))));
}
