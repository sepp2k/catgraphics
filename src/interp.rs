use ast::*;
use std::vec::Vec;
use std::f64::consts::PI;

type Coord = (f64, f64);

pub fn interp<F: FnMut(Coord, Coord) -> ()>(stmnts: &Vec<Stmnt>, mut draw_line: F) -> () {
    let mut coord: Coord = (0.0, 0.0);
    let mut angle: f64 = 0.0;
    let mut pen_down = false;
    for stmnt in stmnts {
        match *stmnt {
            Stmnt::Move(ref length) => {
                let length = eval(&length);
                let new_coord = (coord.0 + length * angle.cos(), coord.1 + length * angle.sin());
                if pen_down {
                    println!("Drawing line from {:?} to {:?}.", coord, new_coord);
                    draw_line(coord, new_coord);
                } else {
                    println!("Moving from {:?} to {:?}.", coord, new_coord);
                }
                coord = new_coord
            },
            Stmnt::Rotate(ref degrees) => {
                let radians = eval(degrees) * PI / 180.0;
                angle += radians;
            },
            Stmnt::PenUp => {
                if !pen_down {
                    println!("Warning: Pen was already up.");
                }
                pen_down = false;
            }
            Stmnt::PenDown => {
                if pen_down {
                    println!("Warning: Pen was already down.");
                }
                pen_down = true;
            }
        }
    }
}

pub fn eval(exp: &Exp) -> f64 {
    match *exp {
        Exp::Const(f) => f,
        Exp::InfixExp(InfixOp::Add, ref lhs, ref rhs) => eval(&*lhs) + eval(&*rhs),
        Exp::InfixExp(InfixOp::Mul, ref lhs, ref rhs) => eval(&*lhs) * eval(&*rhs),
        Exp::InfixExp(InfixOp::Div, ref lhs, ref rhs) => eval(&*lhs) / eval(&*rhs)
    }
}
