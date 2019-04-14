use crate::types::{CastlingRights, Color, FenStage, File, PieceType, Rank, Square};

// Starting Position FEN
pub const INITIAL_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";


// Ranks
pub const RANK_1: Rank = Rank(0);
pub const RANK_2: Rank = Rank(1);
pub const RANK_3: Rank = Rank(2);
pub const RANK_4: Rank = Rank(3);
pub const RANK_5: Rank = Rank(4);
pub const RANK_6: Rank = Rank(5);
pub const RANK_7: Rank = Rank(6);
pub const RANK_8: Rank = Rank(7);
pub const RANKS: [Rank; 8] = [RANK_1, RANK_2, RANK_3, RANK_4, RANK_5, RANK_6, RANK_7, RANK_8];


// Files
pub const FILE_A: File = File(0);
pub const FILE_B: File = File(1);
pub const FILE_C: File = File(2);
pub const FILE_D: File = File(3);
pub const FILE_E: File = File(4);
pub const FILE_F: File = File(5);
pub const FILE_G: File = File(6);
pub const FILE_H: File = File(7);
pub const FILES: [File; 8] = [FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H];


// White base rank squares
pub const A1: Square = Square(0);
pub const B1: Square = Square(1);
pub const C1: Square = Square(2);
pub const D1: Square = Square(3);
pub const E1: Square = Square(4);
pub const F1: Square = Square(5);
pub const G1: Square = Square(6);
pub const H1: Square = Square(7);
pub const A8: Square = Square(56);
pub const B8: Square = Square(57);
pub const C8: Square = Square(58);
pub const D8: Square = Square(59);
pub const E8: Square = Square(60);
pub const F8: Square = Square(61);
pub const G8: Square = Square(62);
pub const H8: Square = Square(63);
pub const SQUARE_INVALID: Square = Square(64);
pub const SQUARES: [Square; 64] = [
    Square(0), Square(1), Square(2), Square(3), Square(4), Square(5), Square(6), Square(7),
    Square(8), Square(9), Square(10), Square(11), Square(12), Square(13), Square(14), Square(15),
    Square(16), Square(17), Square(18), Square(19), Square(20), Square(21), Square(22), Square(23),
    Square(24), Square(25), Square(26), Square(27), Square(28), Square(29), Square(30), Square(31),
    Square(32), Square(33), Square(34), Square(35), Square(36), Square(37), Square(38), Square(39),
    Square(40), Square(41), Square(42), Square(43), Square(44), Square(45), Square(46), Square(47),
    Square(48), Square(49), Square(50), Square(51), Square(52), Square(53), Square(54), Square(55),
    Square(56), Square(57), Square(58), Square(59), Square(60), Square(61), Square(62), Square(63),
];


// Colors
pub const WHITE: Color = Color(0);
pub const BLACK: Color = Color(1);
pub const US: Color = Color(0);
pub const THEM: Color = Color(1);
pub const COLORS: [Color; 2] = [WHITE, BLACK];


// Piece types
pub const PAWN: PieceType = PieceType(0);
pub const KNIGHT: PieceType = PieceType(1);
pub const BISHOP: PieceType = PieceType(2);
pub const ROOK: PieceType = PieceType(3);
pub const QUEEN: PieceType = PieceType(4);
pub const KING: PieceType = PieceType(5);
pub const PIECE_TYPES: [PieceType; 6] = [PAWN, KNIGHT, BISHOP, ROOK, QUEEN, KING];
pub const PIECE_NONE: PieceType = PieceType(6);


// Castling rights
pub const CASTLING_RIGHT_NONE: CastlingRights = CastlingRights(0);
pub const WHITE_KING_SIDE: CastlingRights = CastlingRights(1);
pub const WHITE_QUEEN_SIDE: CastlingRights = CastlingRights(2);
pub const BLACK_KING_SIDE: CastlingRights = CastlingRights(4);
pub const BLACK_QUEEN_SIDE: CastlingRights = CastlingRights(8);


// Move types
pub const MOVE_NORMAL: u32 = 1 << 12;
pub const MOVE_CAPTURE: u32 = 1 << 13;
pub const MOVE_DOUBLE_PUSH: u32 = 1 << 14;
pub const MOVE_ENPASSANT: u32 = 1 << 15;
pub const MOVE_CASTLING: u32 = 1 << 16;
pub const MOVE_PROM_CAP: u32 = 1 << 17;
pub const MOVE_PROMOTION: u32 = 1 << 18;
pub const MOVE_MASK_CAPTURE: u32 = MOVE_CAPTURE | MOVE_ENPASSANT | MOVE_PROM_CAP;
pub const MOVE_MASK_ALL: u32 = MOVE_NORMAL | MOVE_CASTLING | MOVE_ENPASSANT
    | MOVE_PROMOTION | MOVE_DOUBLE_PUSH | MOVE_CAPTURE | MOVE_PROM_CAP;


// Promotion types
pub const PROM_NONE: u32 = 0;
pub const PROM_SHIFT: u32 = 19;
pub const PROM_KNIGHT: u32 = ((KNIGHT.0 as u32) << PROM_SHIFT) as u32;
pub const PROM_BISHOP: u32 = ((BISHOP.0 as u32) << PROM_SHIFT) as u32;
pub const PROM_ROOK: u32 = ((ROOK.0 as u32) << PROM_SHIFT) as u32;
pub const PROM_QUEEN: u32 = ((QUEEN.0 as u32) << PROM_SHIFT) as u32;


// Capture types - only useful if MOVE_CAPTURE bit is set
pub const CAP_SHIFT: u32 = 22;
pub const CAP_PAWN: u32 = ((PAWN.0 as u32) << CAP_SHIFT) as u32;
pub const CAP_KNIGHT: u32 = ((KNIGHT.0 as u32) << CAP_SHIFT) as u32;
pub const CAP_BISHOP: u32 = ((BISHOP.0 as u32) << CAP_SHIFT) as u32;
pub const CAP_ROOK: u32 = ((ROOK.0 as u32) << CAP_SHIFT) as u32;
pub const CAP_QUEEN: u32 = ((QUEEN.0 as u32) << CAP_SHIFT) as u32;


// FEN stages
pub const FEN_STAGES: [FenStage; 7] = [
    FenStage::Pieces,
    FenStage::SideToMove,
    FenStage::CastlingRights,
    FenStage::EnpassantSquare,
    FenStage::HalfMoves,
    FenStage::FullMoves,
    FenStage::Done,
];

pub const CASTLING_SPOILERS: [u8; 64] = [
    13, 15, 15, 15, 12, 15, 15, 14,
    15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15,
     7, 15, 15, 15,  3, 15, 15, 11
];
