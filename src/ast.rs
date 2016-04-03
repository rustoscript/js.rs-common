use std::fmt::{Display, Error, Formatter};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BinOp {
    And,
    Ge,
    Gt,
    Eql,       // ==
    EqlStrict, // ===
    Le,
    Lt,
    Minus,
    Neq,       // !=
    NeqStrict, // !==
    Or,
    Plus,
    Slash,
    Star,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    Const = 110,
    Sign = 100,
    Inc = 90,
    Mult = 70,
    Add = 60,
    Equality = 50,
    And = 40,
    Or = 30,
}

impl BinOp {
    pub fn precedence(&self) -> Precedence {
        match *self {
            BinOp::And => Precedence::And,
            BinOp::Ge | BinOp::Gt | BinOp::Eql | BinOp::Le | BinOp::Lt |
                BinOp::Neq | BinOp::EqlStrict | BinOp::NeqStrict
                => Precedence::Equality,
            BinOp::Or => Precedence::Or,
            BinOp::Minus | BinOp::Plus => Precedence::Add,
            BinOp::Slash | BinOp::Star => Precedence::Mult,
        }
    }

    pub fn is_commutative(&self) -> bool {
        match *self {
            BinOp::Minus | BinOp::Slash => false,
            _ => true
        }
    }
}

impl Display for BinOp {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            BinOp::And => write!(fmt, "&&"),
            BinOp::Ge => write!(fmt, ">="),
            BinOp::Gt => write!(fmt, ">"),
            BinOp::Eql => write!(fmt, "=="),
            BinOp::EqlStrict => write!(fmt, "==="),
            BinOp::Le => write!(fmt, "<="),
            BinOp::Lt => write!(fmt, "<"),
            BinOp::Minus => write!(fmt, "-"),
            BinOp::Neq => write!(fmt, "!="),
            BinOp::NeqStrict => write!(fmt, "!=="),
            BinOp::Or => write!(fmt, "||"),
            BinOp::Plus => write!(fmt, "+"),
            BinOp::Slash => write!(fmt, "/"),
            BinOp::Star => write!(fmt, "*"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Exp {
    BinExp(Box<Exp>, BinOp, Box<Exp>),
    Bool(bool),
    Call(Box<Exp>, Vec<Box<Exp>>),
    Defun(Option<String>, Vec<String>, Vec<Stmt>),
    Float(f64),
    InstanceVar(Box<Exp>, String),
    Neg(Box<Exp>),
    Null,
    NewObject(Box<Exp>, Vec<Box<Exp>>),
    Object(Vec<(String, Box<Exp>)>),
    Pos(Box<Exp>),
    PostDec(Box<Exp>),
    PostInc(Box<Exp>),
    PreDec(Box<Exp>),
    PreInc(Box<Exp>),
    TypeOf(Box<Exp>),
    Str(String),
    Undefined,
    Var(String),
}

impl Exp {
    pub fn precedence(&self) -> Precedence {
        match *self {
            Exp::BinExp(_, ref o, _) => o.precedence(),
            Exp::Neg(_) | Exp::Pos(_) => Precedence::Sign,
            Exp::PostDec(_) | Exp::PostInc(_) | Exp::PreDec(_) | Exp::PreInc(_) => Precedence::Inc,
            _ => Precedence::Const
        }
    }
}

macro_rules! group {
    ($e:expr, $p:expr) => {
        if $e.precedence() < $p {
            format!("({})", $e)
        } else {
            format!("{}", $e)
        }
    }
}

impl Exp {
    fn fmt_helper(&self, mut fmt: &mut Formatter, indent_level: i32) -> Result<(), Error> {
        macro_rules! stmt_block {
            ($stmt:expr) => {
                for s in $stmt {
                    try!(s.fmt_helper(&mut fmt, indent_level + 2))
                }
            }
        }

        match *self {
            Exp::BinExp(ref e1, ref o, ref e2) => {
                let prec = self.precedence();

                // Put grouping parentheses if the left subexpression has a lower-precedence
                // operator, e.g. (1 + 2) * 3
                let left = if prec > e1.precedence() {
                    format!("({})", e1)
                } else {
                    format!("{}", e1)
                };

                let right_prec = e2.precedence();

                // Put grouping parentheses around the right subexpression if it has a
                // lower-precedence operator,  __OR__ if `o` is not commutative and the precedence
                // is the same, e.g. (1 + 2) * 3 __OR__ 1 - (2 + 3)
                let right = if prec > right_prec || (!o.is_commutative() && prec == right_prec) {
                    format!("({})", e2)
                } else {
                    format!("{}", e2)
                };

                write!(fmt, "{} {} {}", left, o, right)
            }
            Exp::Bool(b) => write!(fmt, "{}", b),
            Exp::Call(ref func, ref args) => {
                try!(write!(fmt, "{}(", func));

                for (i, arg) in args.iter().enumerate() {
                    if i != 0 {
                        try!(write!(fmt, ", "));
                    }

                    try!(write!(fmt, "{}", arg));
                }

                write!(fmt, ")")
            }
            Exp::Defun(ref opt, ref params, ref body) => {
                try!(write!(fmt, "function"));

                if let &Some(ref func) = opt {
                    try!(write!(fmt, " {}", func));
                }

                try!(write!(fmt, "("));

                for (i, param) in params.iter().enumerate() {
                    if i != 0 {
                        try!(write!(fmt, ", "));
                    }

                    try!(write!(fmt, "{}", param));
                }

                try!(write!(fmt, ") {{\n"));
                stmt_block!(body);

                let indent : String = (0..indent_level).map(|_| " ").collect();

                write!(fmt, "{}}}", indent)
            }
            Exp::Float(f) => write!(fmt, "{}", f),
            Exp::InstanceVar(ref obj, ref name) => write!(fmt, "{}.{}", obj, name),
            Exp::Neg(ref e) => write!(fmt, "-{}", group!(e, Precedence::Sign)),
            Exp::NewObject(ref name, ref args) => {
                try!(write!(fmt, "new {}(", name));

                for (i, arg) in args.iter().enumerate() {
                    if i != 0 {
                        try!(write!(fmt, ", "));
                    }

                    try!(write!(fmt, "{}", arg))
                }

                write!(fmt, ")")
            }
            Exp::Null => write!(fmt, "null"),
            Exp::Object(ref properties) => {
                if properties.is_empty() {
                    return write!(fmt, "{{}}");
                }

                try!(writeln!(fmt, "{{"));

                let indent : String = (0..indent_level + 2).map(|_| " ").collect();

                for (i, &(ref name, ref prop)) in properties.iter().enumerate() {
                    if i != 0 {
                        try!(writeln!(fmt, ","));
                    }

                    try!(write!(fmt, "{}{}: ", indent, name));
                    try!(prop.fmt_helper(&mut fmt, indent_level + 2));
                }

                write!(fmt, "\n}}")
            }
            Exp::Pos(ref e) => write!(fmt, "+{}", group!(e, Precedence::Sign)),
            Exp::PostDec(ref e) => write!(fmt, "{}--", group!(e, Precedence::Inc)),
            Exp::PostInc(ref e) => write!(fmt, "{}++", group!(e, Precedence::Inc)),
            Exp::PreDec(ref e) => write!(fmt, "--{}", group!(e, Precedence::Inc)),
            Exp::PreInc(ref e) => write!(fmt, "++{}", group!(e, Precedence::Inc)),
            Exp::TypeOf(ref e) => write!(fmt, "typeof {}", e),
            Exp::Str(ref s) => write!(fmt, "\"{}\"", s),
            Exp::Undefined => write!(fmt, "undefined"),
            Exp::Var(ref v) => write!(fmt, "{}", v),
        }
    }
}

impl Display for Exp {
    fn fmt(&self, mut fmt: &mut Formatter) -> Result<(), Error> {
        self.fmt_helper(&mut fmt, 0)
    }
}

#[derive(Clone, Debug)]
pub enum Stmt {
    Assign(String, Exp),
    BareExp(Exp),
    Decl(String, Exp),
    Empty,
    If(Exp, Vec<Stmt>, Vec<Stmt>),
    Ret(Exp),
    Seq(Box<Stmt>, Box<Stmt>),
    Try(Vec<Stmt>, Vec<Stmt>, Vec<Stmt>),
    Throw(Box<Exp>),
    While(Exp, Vec<Stmt>),
}

impl Stmt {
    pub fn fmt_helper(&self, mut fmt: &mut Formatter, indent_level: i32) -> Result<(), Error> {
        macro_rules! stmt_block {
            ($stmt:expr) => {
                for s in $stmt {
                    try!(s.fmt_helper(&mut fmt, indent_level + 2))
                }
            }
        }

        macro_rules! indented_stmt {
            ($s:expr) => {
                try!($s.fmt_helper(&mut fmt, indent_level + 2))
            }
        }

        macro_rules! exp_semi {
            ($exp:expr) => {{
                try!($exp.fmt_helper(&mut fmt, indent_level));
                writeln!(fmt, ";")
            }}
        }

        let indent : String = (0..indent_level).map(|_| " ").collect();

        match *self {
            Stmt::Assign(ref v, ref exp) => {
                try!(write!(fmt, "{}{} = ", indent, v));
                exp_semi!(exp)
            }
            Stmt::BareExp(ref exp) => {
                try!(write!(fmt, "{}", indent));
                exp_semi!(exp)
            }
            Stmt::Decl(ref v, ref exp) => {
                try!(write!(fmt, "{}var {} = ", indent, v));
                exp_semi!(exp)
            }
            Stmt::Empty => Ok(()),
            Stmt::If(ref e, ref s, ref els) => {
                try!(write!(fmt, "{}if (", indent));
                try!(e.fmt_helper(&mut fmt, indent_level + 2));
                try!(writeln!(fmt, ") {{\n"));
                stmt_block!(s);

                if els.len() > 0 {
                    stmt_block!(s);
                }

                //if let &Some(ref stmt) = els {
                //    try!(write!(fmt, "{}else {{\n", indent));
                //    indented_stmt!(stmt);
                //    try!(write!(fmt, "{}}}\n", indent));
                //}

                Ok(())
            }
            Stmt::Ret(ref e) => {
                try!(write!(fmt, "{}return ", indent));
                exp_semi!(e)
            }
            Stmt::Seq(ref s1, ref s2) => {
                try!(s1.fmt_helper(&mut fmt, indent_level));
                s2.fmt_helper(&mut fmt, indent_level)
            }
            Stmt::Throw(ref e) => {
                write!(fmt, "{}throw {}", indent, e)
            }
            Stmt::Try(ref stmt, ref catch_block, ref finally_block) => {
                try!(write!(fmt, "{}try {{\n", indent));
                stmt_block!(stmt);
                try!(write!(fmt, "{}}}\n", indent));

                if catch_block.len() > 0 {
                    try!(write!(fmt, "{}catch {{\n", indent));
                    stmt_block!(catch_block);
                    try!(write!(fmt, "{}}}\n", indent));
                }

                if finally_block.len() > 0 {
                    try!(write!(fmt, "{}finally {{\n", indent));
                    stmt_block!(finally_block);
                    try!(write!(fmt, "{}}}\n", indent));
                }

                Ok(())
            }
            Stmt::While(ref exp, ref stmt) => {
                try!(write!(fmt, "{}while ({}) {{\n", indent, exp));
                stmt_block!(stmt);
                write!(fmt, "{}}}\n", indent)
            }
        }
    }
}

impl Display for Stmt {
    fn fmt(&self, mut fmt: &mut Formatter) -> Result<(), Error> {
        self.fmt_helper(&mut fmt, 0)
    }
}
