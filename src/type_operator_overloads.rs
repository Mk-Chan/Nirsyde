use std::ops;
use crate::types::{Bitboard, Square, CastlingRight};
use crate::types::FenStage::CastlingRights;

impl ops::BitXor<CastlingRight> for CastlingRight {
    type Output = CastlingRight;

    fn bitxor(self, rhs: CastlingRight) -> Self::Output {
        CastlingRight(self.0 ^ rhs.0)
    }
}

impl ops::BitXorAssign<CastlingRight> for CastlingRight {
    fn bitxor_assign(&mut self, rhs: CastlingRight) {
        self.0 ^= rhs.0;
    }
}

impl ops::Add<Square> for Square {
    type Output = Square;

    fn add(self, rhs: Square) -> Self::Output {
        Square(self.0 + rhs.0)
    }
}

impl ops::AddAssign<Square> for Square {
    fn add_assign(&mut self, rhs: Square) {
        self.0 += rhs.0;
    }
}

impl ops::Sub<Square> for Square {
    type Output = Square;

    fn sub(self, rhs: Square) -> Self::Output {
        Square(self.0 + rhs.0)
    }
}

impl ops::SubAssign<Square> for Square {
    fn sub_assign(&mut self, rhs: Square) {
        self.0 -= rhs.0;
    }
}

impl ops::BitAnd<Bitboard> for Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl ops::BitAndAssign<Bitboard> for Bitboard {
    fn bitand_assign(&mut self, rhs: Bitboard) {
        self.0 &= rhs.0;
    }
}

impl ops::BitOr<Bitboard> for Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl ops::BitOrAssign<Bitboard> for Bitboard {
    fn bitor_assign(&mut self, rhs: Bitboard) {
        self.0 |= rhs.0;
    }
}

impl ops::BitXor<Bitboard> for Bitboard {
    type Output = Bitboard;

    fn bitxor(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl ops::BitXorAssign<Bitboard> for Bitboard {
    fn bitxor_assign(&mut self, rhs: Bitboard) {
        self.0 ^= rhs.0;
    }
}
