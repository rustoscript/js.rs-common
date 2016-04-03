#[macro_export]
macro_rules! exp {
    ($e1:expr, $o:expr, $e2:expr) => { BinExp(Box::new($e1), $o, Box::new($e2)) }
}

#[macro_export]
macro_rules! post_dec {
    ($e:expr) => { PostDec(Box::new($e)) }
}

#[macro_export]
macro_rules! post_inc {
    ($e:expr) => { PostInc(Box::new($e)) }
}

#[macro_export]
macro_rules! pre_dec {
    ($e:expr) => { PreDec(Box::new($e)) }
}

#[macro_export]
macro_rules! pre_inc {
    ($e:expr) => { PreInc(Box::new($e)) }
}

#[macro_export]
macro_rules! neg_var {
    ($e:expr) => { Neg(Box::new(var!($e))) }
}

#[macro_export]
macro_rules! pos_var {
    ($e:expr) => { Pos(Box::new(var!($e))) }
}

#[macro_export]
macro_rules! instance_var {
    ($o:expr, $name:expr) => { InstanceVar(Box::new($o), String::from($name)) }
}

#[macro_export]
macro_rules! call {
    ($name:expr, $args:expr) => {
        Call(Box::new($name), $args.into_iter().map(|x| Box::new(x)).collect())
    }
}

#[macro_export]
macro_rules! method {
    ($o:expr, $name:expr, [$($arg:expr),*]) => {
        Method(Box::new($o), String::from($name), vec![$(Box::new($arg)),*])
    }
}

#[macro_export]
macro_rules! assign {
    ($v:expr, $e:expr) => { Assign(String::from($v), $e) }
}

#[macro_export]
macro_rules! decl {
    ($v:expr, $e:expr) => { Decl(String::from($v), $e) }
}

#[macro_export]
macro_rules! seq {
    ($s1:expr, $s2:expr) => { Seq(Box::new($s1), Box::new($s2)) }
}

#[macro_export]
macro_rules! var {
    ($s:expr) => { Var(String::from($s)) }
}


#[macro_export]
macro_rules! defun {
    (($($param:expr),*) $stmt:expr) => {
        Defun(
            None,
            vec![
                $(String::from($param)),*
            ],
            vec![$stmt]
        )
    };

    ($name:expr, ($($param:expr),*) $stmt:expr) => {
        Defun(
            Some(String::from($name)),
            vec![
                $(String::from($param)),*
            ],
            vec![$stmt]
        )
    }
}

#[macro_export]
macro_rules! new_obj {
    ($name:expr) => { NewObject(Box::new($name), vec![]) };

    ($name:expr, $($arg:expr),*) => {
        NewObject(Box::new($name), vec![
            $(Box::new($arg)),*
        ])
    }
}


#[macro_export]
macro_rules! obj {
    ($($name:expr => $prop:expr),*) => {
        Object(vec![
            $((String::from($name), Box::new($prop))),*
        ])
    }
}

#[macro_export]
macro_rules! parse_exp {
    ($s:expr) => { parse_Exp($s).unwrap() }
}

#[macro_export]
macro_rules! parse_stmt {
    ($s:expr) => { parse_Stmt($s).unwrap() }
}
