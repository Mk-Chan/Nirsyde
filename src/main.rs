mod types;
mod utils;
mod castling;
mod position;

use crate::utils::Printable;

fn main() {
    println!("Hello, world!");
    let x: u8 = 1;
    println!("{}", x);
    let y: u8 = !x;
    println!("{}", y);
    let z: types::Bitboard = 0x00ff00000000ff00;
    utils::print_bitboard(z);

    let mut cr = castling::CastlingRights{ rights: 0b0000 };
    cr.grant(castling::CastlingRightType::BlackKingSide);
    cr.grant(castling::CastlingRightType::WhiteKingSide);
    cr.print();
    cr.revoke(castling::CastlingRightType::WhiteKingSide);
    cr.print();

    types::Square::A2.print();
}
