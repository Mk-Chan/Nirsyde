#![allow(dead_code)]

use crate::constants::*;
use crate::position::Position;

mod position;
mod types;
mod constants;
mod type_trait_impls;
mod type_traits;
mod type_operator_overloads;

fn main() {
    let pos = Position::from(INITIAL_FEN);
    pos.print_parts();
}
