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
