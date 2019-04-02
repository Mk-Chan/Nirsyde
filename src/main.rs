#![allow(dead_code)]

use crate::constants::*;
use crate::position::Position;
use crate::types::Move;
use crate::type_traits::Printable;

mod position;
mod types;
mod constants;
mod type_trait_impls;
mod type_traits;
mod type_operator_overloads;

fn main() {
    let pos = Position::from(INITIAL_FEN);
    pos.print_parts();
    Move::capture(A1, A8, CAP_PAWN).from_square().print();
    Move::capture(A1, A8, CAP_PAWN).to_square().print();
    Move::promotion_capture(A1, A8, PROM_ROOK, CAP_QUEEN).promotion_type().print();
    Move::promotion_capture(B8, A8, PROM_ROOK, CAP_QUEEN).capture_type().print();
    pos.flipped().print_parts();
}
