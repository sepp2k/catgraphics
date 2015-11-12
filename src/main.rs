extern crate piston_window;

mod ast;
mod ast_display;
mod interp;

use piston_window::*;
use ast::*;
use interp::*;

fn main() {
    let width = 640;
    let height = 480;
    let c_x = width as f64 / 2.0;
    let c_y = height as f64 / 2.0;
    let scale = 2.0;
    let window: PistonWindow = WindowSettings::new("Cat Graphics :-)", [width, height])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    let exp = mk_op(InfixOp::Add,
                    c(4.0),
                    mk_op(InfixOp::Div,
                          mk_op(InfixOp::Mul, c(3.0), c(2.0)),
                          c(1.0)));
    println!("{} = {}", exp, eval(&*exp));

    let prog = vec![
        Stmnt::Rotate(c(90.0)),
        Stmnt::PenDown,
        Stmnt::Move(exp),
        Stmnt::Rotate(c(13.0)),
        Stmnt::Move(c(42.0)),
        Stmnt::PenUp];
    println!("Running program:");
    ast_display::print_stmnts(&prog);

    let white = [1.0; 4];
    let black = [0.0, 0.0, 0.0, 1.0];
    for e in window {
        e.draw_2d(|c, g| {
            clear(white, g);
            interp(&prog, |from, to| {
                let coords = [from.0 * scale + c_x, from.1 * scale + c_y,
                              to.0 *scale + c_x, to.1 * scale + c_y];
                line(black, 2.0, coords, c.transform, g);
            });
        });
    }
}
