use ast::*;

pub fn eval(exp: &Exp)->i32 {
    match *exp {
        Exp::ConstInt(i) => i,
        Exp::InfixExp(InfixOp::Add, ref lhs, ref rhs) => eval(&*lhs) + eval(&*rhs),
        Exp::InfixExp(InfixOp::Mul, ref lhs, ref rhs) => eval(&*lhs) * eval(&*rhs),
        Exp::InfixExp(InfixOp::Div, ref lhs, ref rhs) => eval(&*lhs) / eval(&*rhs)
    }
}
