use std::fmt::{Debug, Error, Formatter};

pub enum Expression {
    Integer(i64),
    Operation(Operator, Box<Expression>, Box<Expression>),
}

pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Debug for Expression {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        use self::Expression::*;

        match self {
            Integer(n) => write!(fmt, "{:?}", n),
            Operation(op, ref l, ref r) => write!(fmt, "({:?} {:?} {:?})", op, l, r),
        }
    }
}

impl Debug for Operator {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        use self::Operator::*;

        match self {
            Mul => write!(fmt, "mul"),
            Div => write!(fmt, "div"),
            Add => write!(fmt, "add"),
            Sub => write!(fmt, "sub"),
        }
    }
}
