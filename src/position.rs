use crate::constants::*;
use crate::type_traits::Printable;
use crate::types::{Bitboard, CastlingRights, Color, ColoredPiece, FenStage, Move, PieceType, Square};

#[derive(Copy, Clone)]
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
                        position.piece_types[piece_type.0 as usize] |= Bitboard::from(&curr_square);
                        position.colors[color.0 as usize] |= Bitboard::from(&curr_square);
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

        if self.enpassant_sq != SQUARE_INVALID {
            self.enpassant_sq.0 ^= 56;
        }

        let tmp_cr = (self.castling_rights.0 & 3) << 2;
        self.castling_rights.0 >>= 2;
        self.castling_rights.0 ^= tmp_cr;

        self.side_to_move = !self.side_to_move;
    }

    pub fn flipped(&self) -> Position {
        let mut pos = self.clone();
        pos.flip();
        pos
    }

    pub fn make_move(&self, m: &Move) -> Position {
        let mut pos = self.clone();
        let from_sq = m.from_square();
        let to_sq = m.to_square();
        let moving_pt = pos.piece_type_on(&from_sq);

        pos.castling_rights.spoil(CastlingRights(CASTLING_SPOILERS[from_sq.0 as usize]));
        pos.castling_rights.spoil(CastlingRights(CASTLING_SPOILERS[to_sq.0 as usize]));

        if moving_pt == PAWN {
            pos.halfmoves = 0;
        }

        if m.is_move_type(MOVE_NORMAL) {
            pos.move_piece(&from_sq, &to_sq, &moving_pt, &US);
        } else if m.is_move_type(MOVE_CAPTURE) {
            let captured_pt = pos.piece_type_on(&to_sq);
            pos.toggle_piece(&to_sq, &captured_pt, &THEM);
            pos.move_piece(&from_sq, &to_sq, &moving_pt, &US);
            pos.halfmoves = 0;
        } else if m.is_move_type(MOVE_DOUBLE_PUSH) {
            pos.move_piece(&from_sq, &to_sq, &moving_pt, &US);
            pos.enpassant_sq = to_sq - Square(8);
        } else if m.is_move_type(MOVE_ENPASSANT) {
            pos.move_piece(&from_sq, &to_sq, &moving_pt, &US);
            let ep_sq = pos.enpassant_sq;
            pos.toggle_piece(&ep_sq, &PAWN, &THEM);
        } else if m.is_move_type(MOVE_CASTLING) {
            let (rfrom_sq, rto_sq) = match to_sq {
                C1 => (A1, D1),
                G1 => (H1, F1),
                _ => panic!("Unknown castling to square"),
            };
            pos.toggle_piece(&rfrom_sq, &ROOK, &US);
            pos.toggle_piece(&from_sq, &KING, &US);
            pos.toggle_piece(&rto_sq, &ROOK, &US);
            pos.toggle_piece(&to_sq, &KING, &US);
        } else if m.is_move_type(MOVE_PROM_CAP) {
            let captured_pt = pos.piece_type_on(&to_sq);
            let prom_type = m.promotion_type();
            pos.toggle_piece(&to_sq, &captured_pt, &THEM);
            pos.toggle_piece(&from_sq, &PAWN, &US);
            pos.toggle_piece(&to_sq, &prom_type, &US);
        } else if m.is_move_type(MOVE_PROMOTION) {
            let prom_type = m.promotion_type();
            pos.toggle_piece(&from_sq, &PAWN, &US);
            pos.toggle_piece(&to_sq, &prom_type, &US);
        }

        if !m.is_move_type(MOVE_DOUBLE_PUSH) {
            pos.enpassant_sq = SQUARE_INVALID;
        }

        pos.flip();
        pos
    }

    pub fn piece_type_on(&self, sq: &Square) -> PieceType {
        let sq_bb = Bitboard::from(sq);
        for pt in PIECE_TYPES.iter() {
            if self.piece_types[pt.0 as usize] & sq_bb != Bitboard(0) {
                return *pt;
            }
        }
        PIECE_NONE
    }

    fn move_piece(&mut self, from: &Square, to: &Square, pt: &PieceType, color: &Color) {
        let from_to_mask = Bitboard::from(from) ^ Bitboard::from(to);
        self.colors[color.0 as usize] ^= from_to_mask;
        self.piece_types[pt.0 as usize] ^= from_to_mask;
    }

    fn toggle_piece(&mut self, sq: &Square, pt: &PieceType, color: &Color) {
        let toggle_mask = Bitboard::from(sq);
        self.colors[color.0 as usize] ^= toggle_mask;
        self.piece_types[pt.0 as usize] ^= toggle_mask;
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
