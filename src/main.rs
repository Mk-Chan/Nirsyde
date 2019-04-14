#![allow(dead_code)]

use crate::constants::*;
use crate::position::Position;
use crate::type_traits::Printable;
use crate::types::{Move, Square};

mod position;
mod types;
mod constants;
mod type_trait_impls;
mod type_traits;
mod type_operator_overloads;

fn main() {
    let pos = Position::from(INITIAL_FEN);
    let npos = pos.make_move(&Move::normal(&Square(1), &Square(18)));
    npos.print_parts();
}
