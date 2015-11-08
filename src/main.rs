mod ast;
mod eval;

use ast::*;
use eval::*;

fn main() {
    let exp = mk_op(InfixOp::Add,
                   c(4.0),
                   mk_op(InfixOp::Div,
                        mk_op(InfixOp::Mul, c(3.0), c(2.0)),
                        c(1.0)));
    println!("{} = {}", exp, eval(&*exp));
}
