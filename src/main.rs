enum Exp {
    ConstInt(i32),
    InfixExp(InfixOp, Box<Exp>, Box<Exp>)
}

enum InfixOp {
    Add, Mul, Div
}

fn eval(exp: &Exp)->i32 {
    match *exp {
        Exp::ConstInt(i) => i,
        Exp::InfixExp(InfixOp::Add, ref lhs, ref rhs) => eval(&*lhs) + eval(&*rhs),
        Exp::InfixExp(InfixOp::Mul, ref lhs, ref rhs) => eval(&*lhs) * eval(&*rhs),
        Exp::InfixExp(InfixOp::Div, ref lhs, ref rhs) => eval(&*lhs) / eval(&*rhs)
    }
}

fn c(i: i32) -> Box<Exp> {
    Box::new(Exp::ConstInt(i))
}

fn mk_op(op: InfixOp, lhs: Box<Exp>, rhs: Box<Exp>) -> Box<Exp> {
    Box::new(Exp::InfixExp(op, lhs, rhs))
}

fn main() {
    let exp = mk_op(InfixOp::Add,
                   c(4),
                   mk_op(InfixOp::Div,
                        mk_op(InfixOp::Mul, c(3), c(2)),
                        c(1)));
    println!("4 + 3 * 2 / 1 = {}", eval(&*exp));
}
