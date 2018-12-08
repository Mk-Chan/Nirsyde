use crate::types::Square;
use crate::types::PieceType;
use crate::bitboard::Bitboard;
use crate::bitboard::create_bb;
use crate::castling::CastlingRights;

pub struct Position {
    pieces: [Bitboard; 6],
    colors: [Bitboard; 2],
    enpassant_sq: Square,
    castling_rights: CastlingRights,
    flipped: bool,
    // Record 3-fold repetition with hashes
}

impl Position {
    pub fn new(fen: &str) {
        println!("{}", fen);
        let square = Square::A8;
        for ch in fen.chars() {
            let piece_type = PieceType::parse(ch);
            let piece_index = piece_type.index();
            let piece_bb = create_bb(square.index());
            // TODO: Continue the FEN parser
        }
    }
}
