use ast::*;

pub fn eval(exp: &Exp) -> f64 {
    match *exp {
        Exp::Const(f) => f,
        Exp::InfixExp(InfixOp::Add, ref lhs, ref rhs) => eval(&*lhs) + eval(&*rhs),
        Exp::InfixExp(InfixOp::Mul, ref lhs, ref rhs) => eval(&*lhs) * eval(&*rhs),
        Exp::InfixExp(InfixOp::Div, ref lhs, ref rhs) => eval(&*lhs) / eval(&*rhs)
    }
}
