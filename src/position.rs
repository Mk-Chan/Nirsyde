use crate::types::Bitboard;
use crate::types::Square;
use crate::castling::CastlingRights;

pub struct Position {
    pieces: [Bitboard; 6],
    colors: [Bitboard; 2],
    enpassant_sq: Square,
    castling_rights: CastlingRights,
    flipped: bool,
    // Record 3-fold repetition with hashes
}
