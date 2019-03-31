use crate::constants::*;
use crate::type_traits::Printable;
use crate::types::{Bitboard, CastlingRights, Color, ColoredPiece, FenStage, Square};

pub struct Position {
    piece_types: [Bitboard; 6],
    colors: [Bitboard; 2],
    enpassant_sq: Square,
    castling_rights: CastlingRights,
    side_to_move: Color,
    halfmoves: u8,
    fullmoves: u32,
}

impl From<&String> for Position {
    fn from(fen: &String) -> Self {
        let mut position = Position {
            piece_types: [Bitboard(0); 6],
            colors: [Bitboard(0); 2],
            enpassant_sq: SQUARE_INVALID,
            castling_rights: CASTLING_RIGHT_NONE,
            side_to_move: WHITE,
            halfmoves: 0,
            fullmoves: 1,
        };
        let mut fen_stages = FEN_STAGES.iter();
        let mut stage = fen_stages.next().unwrap();
        let mut curr_square = A8;
        let fen_parts: Vec<String> = fen.split(" ").map(|x| String::from(x)).collect();
        for fen_part in fen_parts.iter() {
            if *stage == FenStage::Pieces {
                for ch in fen_part.chars() {
                    if ch.is_numeric() {
                        curr_square += Square(ch.to_digit(10).unwrap() as u8);
                    } else if ch == '/' {
                        curr_square -= Square(16);
                    } else {
                        let ColoredPiece(piece_type, color) = ColoredPiece::from(&ch);
                        position.piece_types[piece_type.0 as usize] |= Bitboard::from_shift(curr_square.0);
                        position.colors[color.0 as usize] |= Bitboard::from_shift(curr_square.0);
                        curr_square += Square(1);
                    }
                }
            } else if *stage == FenStage::SideToMove {
                let ch = fen_part.chars().next().unwrap();
                position.side_to_move = Color::from(&ch);
            } else if *stage == FenStage::CastlingRights {
                for ch in fen_part.chars() {
                    position.castling_rights ^= CastlingRights::from(&ch);
                }
            } else if *stage == FenStage::EnpassantSquare {
                position.enpassant_sq = Square::from(fen_part);
            } else if *stage == FenStage::HalfMoves {
                position.halfmoves = fen_part.parse::<u8>().unwrap();
            } else if *stage == FenStage::FullMoves {
                position.fullmoves = fen_part.parse::<u32>().unwrap();
            }
            stage = fen_stages.next().unwrap();
        }

        if position.side_to_move == BLACK {
            position.flip();
        }

        debug_assert!(*stage > FenStage::EnpassantSquare);

        position
    }
}

impl From<&str> for Position {
    fn from(fen: &str) -> Self {
        Position::from(&String::from(fen))
    }
}

impl Position {
    pub fn flip(&mut self) {
        for piece_type in PIECE_TYPES.iter() {
            self.piece_types[piece_type.0 as usize] =
                self.piece_types[piece_type.0 as usize].swap_bytes();
        }
        for color in COLORS.iter() {
            self.colors[color.0 as usize] =
                self.colors[color.0 as usize].swap_bytes();
        }

        self.colors.swap(WHITE.0 as usize, BLACK.0 as usize);
    }

    pub fn print_parts(&self) {
        for (i, bb) in self.piece_types.iter().enumerate() {
            PIECE_TYPES[i].print();
            bb.print();
        }
        for (i, bb) in self.colors.iter().enumerate() {
            COLORS[i].print();
            bb.print();
        }
        self.side_to_move.print();
        self.castling_rights.print();
        self.enpassant_sq.print();
        println!("{}", self.halfmoves);
        println!("{}", self.fullmoves);
    }
}
