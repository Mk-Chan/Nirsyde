use crate::constants::{MOVE_PROM_CAP, MOVE_ENPASSANT, MOVE_CAPTURE, MOVE_NORMAL, MOVE_PROMOTION, MOVE_CASTLING, MOVE_DOUBLE_PUSH, CAP_SHIFT, PROM_SHIFT};

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Rank(pub u8);

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct File(pub u8);

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Square(pub u8);

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct PieceType(pub u8);

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Color(pub u8);

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct ColoredPiece(pub PieceType, pub Color);

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct CastlingRights(pub u8);

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Bitboard(pub u64);

// 0-5 bits - encoded from square
// 6-11 bits - encoded to square
// 12-17 bits - flags for move type
// 18-20 bits - encoded promotion type
// 21-23 bits - encoded captured piece type
#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Move(pub u32);

#[derive(Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
pub enum FenStage {
    Pieces,
    SideToMove,
    CastlingRights,
    EnpassantSquare,
    HalfMoves,
    FullMoves,
    Done,
}

impl Move {
    pub fn normal(from: Square, to: Square) -> Move {
        Move((from.0 as u32) | ((to.0 as u32) << 6) | MOVE_NORMAL)
    }

    pub fn capture(from: Square, to: Square, cap_type: u32) -> Move {
        Move((from.0 as u32) | ((to.0 as u32) << 6) | MOVE_CAPTURE | cap_type)
    }

    pub fn promotion(from: Square, to: Square, prom_type: u32, cap_type: u32) -> Move {
        Move((from.0 as u32) | ((to.0 as u32) << 6) | MOVE_PROMOTION | prom_type)
    }

    pub fn double_push(from: Square, to: Square) -> Move {
        Move((from.0 as u32) | ((to.0 as u32) << 6) | MOVE_DOUBLE_PUSH)
    }

    pub fn castle(from: Square, to: Square) -> Move {
        Move((from.0 as u32) | ((to.0 as u32) << 6) | MOVE_CASTLING)
    }

    pub fn promotion_capture(from: Square, to: Square, prom_type: u32, cap_type: u32) -> Move {
        Move((from.0 as u32) | ((to.0 as u32) << 6) | MOVE_PROM_CAP | prom_type | cap_type)
    }

    pub fn enpassant(from: Square, to: Square) -> Move {
        Move((from.0 as u32) | ((to.0 as u32) << 6) | MOVE_ENPASSANT)
    }

    pub fn from_square(&self) -> Square {
        Square((self.0 & 0x3f) as u8)
    }

    pub fn to_square(&self) -> Square {
        Square(((self.0 >> 6) & 0x3f) as u8)
    }

    pub fn promotion_type(&self) -> PieceType {
        PieceType(((self.0 >> PROM_SHIFT) & 7) as u8)
    }

    pub fn capture_type(&self) -> PieceType {
        PieceType((self.0 >> CAP_SHIFT) as u8)
    }

    pub fn is_move_type(&self, move_type: u32) -> bool {
        (self.0 & move_type) != 0
    }
}

impl Bitboard {
    pub fn from_shift(shift: u8) -> Bitboard {
        debug_assert!(shift < 64);
        Bitboard((1 as u64) << shift)
    }

    pub fn swap_bytes(&self) -> Bitboard {
        Bitboard(self.0.swap_bytes())
    }

    pub fn lsb(&self) -> u32 {
        self.0.trailing_zeros()
    }
}

impl Square {
    pub fn new(file: File, rank: Rank) -> Self {
        Square(file.0 + (rank.0 << 3))
    }

    pub fn rank(&self) -> Rank {
        Rank(self.0 >> 3)
    }

    pub fn file(&self) -> File {
        File(self.0 & 7)
    }
}

impl CastlingRights {
    pub fn allows(&self, cr: CastlingRights) -> bool {
        self.0 & cr.0 != 0
    }
}
