use crate::tree::*;
use piston_window::{line, Context, G2d};
use piston_window::types::Color;

pub fn run(con: &Context, g: &mut G2d, tree: &mut Tree){
    if !tree.is_generated {
        tree.generate();
    }
    tree.draw(con, g);
}