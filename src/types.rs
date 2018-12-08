use crate::utils::Printable;

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

// TODO: Move each type into its own respective module
// TODO: Implement index fn for Square which returns the mapping: A1 -> 0 ... H8 -> 63.
// TODO: Implement iterators over each file and rank from starting square reverse and forward (?!)

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum PieceType {
    Pawn, Knight, Bishop, Rook, Queen, King
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum Color {
    White, Black
}

impl PieceType {
    pub fn parse(ch: char) -> PieceType {
        match ch {
            'p' => PieceType::Pawn,
            'n' => PieceType::Knight,
            'b' => PieceType::Bishop,
            'r' => PieceType::Rook,
            'q' => PieceType::Queen,
            'k' => PieceType::King,
        }
    }

    pub fn index(&self) -> i8 {
        match *self {
            PieceType::Pawn => 0,
            PieceType::Knight => 1,
            PieceType::Bishop => 2,
            PieceType::Rook => 3,
            PieceType::Queen => 4,
            PieceType::King => 5,
        }
    }
}

impl Printable for Square {
    fn print(&self) {
        let x = *self;
        println!("{}", x as i32);
    }
}

impl Printable for Color {
    fn print(&self) {
        let ch = match *self {
            Color::White => 'w',
            Color::Black => 'b',
        };
        println!("{}", ch);
    }
}
