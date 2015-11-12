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
