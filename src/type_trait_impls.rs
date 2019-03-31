use crate::constants::*;
use crate::type_traits::Printable;
use crate::types::{Bitboard, CastlingRights, Color, ColoredPiece, File, PieceType, Rank, Square};

impl From<&char> for CastlingRights {
    fn from(c: &char) -> Self {
        match *c {
            'k' => BLACK_KING_SIDE,
            'q' => BLACK_QUEEN_SIDE,
            'K' => WHITE_KING_SIDE,
            'Q' => WHITE_QUEEN_SIDE,
            '-' => CASTLING_RIGHT_NONE,
            _ => panic!("Unknown castling right!"),
        }
    }
}

impl From<&CastlingRights> for char {
    fn from(c: &CastlingRights) -> Self {
        match *c {
            BLACK_KING_SIDE => 'k',
            BLACK_QUEEN_SIDE => 'q',
            WHITE_KING_SIDE => 'K',
            WHITE_QUEEN_SIDE => 'Q',
            CASTLING_RIGHT_NONE => '-',
            _ => panic!("Unknown castling right!"),
        }
    }
}

impl From<&char> for Color {
    fn from(c: &char) -> Self {
        match *c {
            'w' => WHITE,
            'b' => BLACK,
            _ => panic!("Unknown color!"),
        }
    }
}

impl From<&Color> for char {
    fn from(c: &Color) -> Self {
        match *c {
            WHITE => 'w',
            BLACK => 'b',
            _ => panic!("Unknown color!"),
        }
    }
}

impl From<&char> for PieceType {
    fn from(c: &char) -> Self {
        let c_lower = c.to_ascii_lowercase();
        match c_lower {
            'p' => PAWN,
            'n' => KNIGHT,
            'b' => BISHOP,
            'r' => ROOK,
            'q' => QUEEN,
            'k' => KING,
            _ => panic!("Unknown piece type!"),
        }
    }
}

impl From<&PieceType> for char {
    fn from(pt: &PieceType) -> Self {
        match *pt {
            PAWN => 'p',
            KNIGHT => 'n',
            BISHOP => 'b',
            ROOK => 'r',
            QUEEN => 'q',
            KING => 'k',
            _ => panic!("Unknown piece type!"),
        }
    }
}

impl From<&char> for ColoredPiece {
    fn from(c: &char) -> Self {
        let color = if c.is_uppercase() { WHITE } else { BLACK };
        let piece_type = PieceType::from(c);
        ColoredPiece(piece_type, color)
    }
}

impl From<&ColoredPiece> for char {
    fn from(piece: &ColoredPiece) -> Self {
        let piece_type_ch = char::from(&piece.0);
        if piece.1 == WHITE { piece_type_ch.to_ascii_uppercase() } else { piece_type_ch }
    }
}

impl From<&char> for File {
    fn from(f: &char) -> Self {
        match *f {
            'a' => FILE_A,
            'b' => FILE_B,
            'c' => FILE_C,
            'd' => FILE_D,
            'e' => FILE_E,
            'f' => FILE_F,
            'g' => FILE_G,
            'h' => FILE_H,
            _ => panic!("Unknown file!"),
        }
    }
}

impl From<&File> for char {
    fn from(f: &File) -> Self {
        match *f {
            FILE_A => 'a',
            FILE_B => 'b',
            FILE_C => 'c',
            FILE_D => 'd',
            FILE_E => 'e',
            FILE_F => 'f',
            FILE_G => 'g',
            FILE_H => 'h',
            _ => panic!("Unknown file!"),
        }
    }
}

impl From<&char> for Rank {
    fn from(f: &char) -> Self {
        match *f {
            '1' => RANK_1,
            '2' => RANK_2,
            '3' => RANK_3,
            '4' => RANK_4,
            '5' => RANK_5,
            '6' => RANK_6,
            '7' => RANK_7,
            '8' => RANK_8,
            _ => panic!("Unknown rank!"),
        }
    }
}

impl From<&Rank> for char {
    fn from(f: &Rank) -> Self {
        match *f {
            RANK_1 => '1',
            RANK_2 => '2',
            RANK_3 => '3',
            RANK_4 => '4',
            RANK_5 => '5',
            RANK_6 => '6',
            RANK_7 => '7',
            RANK_8 => '8',
            _ => panic!("Unknown rank!"),
        }
    }
}

impl From<&String> for Square {
    fn from(s: &String) -> Self {
        if *s == "-" {
            SQUARE_INVALID
        } else {
            let mut s = s.chars();
            let file = File(s.next().unwrap() as u8 - 'a' as u8);
            let rank = Rank(s.next().unwrap() as u8 - '1' as u8);
            Square::new(file, rank)
        }
    }
}

impl From<&Square> for String {
    fn from(sq: &Square) -> Self {
        if *sq == SQUARE_INVALID {
            String::from("-")
        } else {
            let file_ch = char::from(&sq.file());
            let rank_ch = char::from(&sq.rank());
            [file_ch, rank_ch].iter().collect()
        }
    }
}

impl Printable for CastlingRights {
    fn print(&self) {
        let mut no_castling = true;
        if self.allows(BLACK_KING_SIDE) {
            print!("{}", char::from(&BLACK_KING_SIDE));
            no_castling = false;
        }
        if self.allows(BLACK_QUEEN_SIDE) {
            print!("{}", char::from(&BLACK_QUEEN_SIDE));
            no_castling = false;
        }
        if self.allows(WHITE_KING_SIDE) {
            print!("{}", char::from(&WHITE_KING_SIDE));
            no_castling = false;
        }
        if self.allows(WHITE_QUEEN_SIDE) {
            print!("{}", char::from(&WHITE_QUEEN_SIDE));
            no_castling = false;
        }
        if no_castling {
            print!("-");
        }
        println!();
    }
}

impl Printable for Square {
    fn print(&self) {
        println!("{}", String::from(self))
    }
}

impl Printable for Color {
    fn print(&self) {
        println!("{}", char::from(self));
    }
}

impl Printable for PieceType {
    fn print(&self) {
        println!("{}", char::from(self));
    }
}

impl Printable for ColoredPiece {
    fn print(&self) {
        println!("{}", char::from(self));
    }
}

impl Printable for Bitboard {
    fn print(&self) {
        for shift in 0..64 {
            let bit = (1 as u64) << (shift ^ 56);
            if shift != 0 && shift % 8 == 0 {
                println!();
            }
            if (self.0 & bit) != 0 {
                print!("X ");
            } else {
                print!("- ");
            }
        }
        println!();
    }
}
