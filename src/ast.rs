#[derive (Debug)]
pub enum Stmnt {
    Move(Box<Exp>),
    Rotate(Box<Exp>),
    PenUp,
    PenDown
}

#[derive(Debug)]
pub enum Exp {
    Const(f64),
    InfixExp(InfixOp, Box<Exp>, Box<Exp>)
}

#[derive(Debug, Clone, Copy)]
pub enum InfixOp {
    Add, Mul, Div
}

pub fn c(num: f64) -> Box<Exp> {
    Box::new(Exp::Const(num))
}

pub fn mk_op(op: InfixOp, lhs: Box<Exp>, rhs: Box<Exp>) -> Box<Exp> {
    Box::new(Exp::InfixExp(op, lhs, rhs))
}

use std::fmt;

impl fmt::Display for InfixOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InfixOp::Add => write!(f, "+"),
            InfixOp::Mul => write!(f, "*"),
            InfixOp::Div => write!(f, "/")
        }
    }
}

impl fmt::Display for Exp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Exp::Const(num) => write!(f, "{}", num),
            Exp::InfixExp(op, ref lhs, ref rhs) => write!(f, "({} {} {})", lhs, op, rhs)
        }
    }
}
