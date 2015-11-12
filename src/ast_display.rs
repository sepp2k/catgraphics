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

impl fmt::Display for Stmnt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Stmnt::Move(ref exp) => write!(f, "move {}", exp),
            Stmnt::Rotate(ref exp) => write!(f, "rotate {}", exp),
            Stmnt::PenUp => write!(f, "pen up"),
            Stmnt::PenDown => write!(f, "pen down")
        }
    }
}

pub fn print_stmnts(stmnts: &Vec<Stmnt>) -> () {
    for stmnt in stmnts {
        println!("{}", stmnt)
    }
}
