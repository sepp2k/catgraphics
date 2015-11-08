mod ast;
mod interp;

use ast::*;
use interp::*;

fn main() {
    let exp = mk_op(InfixOp::Add,
                   c(4.0),
                   mk_op(InfixOp::Div,
                        mk_op(InfixOp::Mul, c(3.0), c(2.0)),
                        c(1.0)));
    println!("{} = {}", exp, eval(&*exp));

    let prog = vec![
        Stmnt::Move(c(42.0)),
        Stmnt::Rotate(c(90.0)),
        Stmnt::PenDown,
        Stmnt::Move(exp),
        Stmnt::PenUp,
        Stmnt::Rotate(c(13.0)),
        Stmnt::Move(c(23.0))];
    interp(&prog);
}
