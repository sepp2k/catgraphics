mod ast;
mod eval;

use ast::*;
use eval::*;

fn main() {
    let exp = mk_op(InfixOp::Add,
                   c(4),
                   mk_op(InfixOp::Div,
                        mk_op(InfixOp::Mul, c(3), c(2)),
                        c(1)));
    println!("4 + 3 * 2 / 1 = {}", eval(&*exp));
}
