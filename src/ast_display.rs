use ast::*;
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
