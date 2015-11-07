pub enum Exp {
    ConstInt(i32),
    InfixExp(InfixOp, Box<Exp>, Box<Exp>)
}

pub enum InfixOp {
    Add, Mul, Div
}

pub fn c(i: i32) -> Box<Exp> {
    Box::new(Exp::ConstInt(i))
}

pub fn mk_op(op: InfixOp, lhs: Box<Exp>, rhs: Box<Exp>) -> Box<Exp> {
    Box::new(Exp::InfixExp(op, lhs, rhs))
}
