pub type Bitboard = u64;

pub fn print_bitboard(bb: Bitboard) {
    for shift in 0..64 {
        let bit = (1 as u64) << (shift ^ 56);
        if shift != 0 && shift % 8 == 0 {
            println!();
        }
        if (bb & bit) != 0 {
            print!("X ");
        }
            else {
                print!("- ");
            }
    }
    println!();
}

pub fn create_bb(bit: i8) -> Bitboard {
    assert!(bit > 0 && bit < 64);
    (1 << bit) as Bitboard
}

pub fn set_bit(bb: &mut Bitboard, bit: i8) {
    assert!(bit > 0 && bit < 64);
    bb |= bit;
}

pub fn clear_bit(bb: &mut Bitboard, bit: i8) {
    assert!(bit > 0 && bit < 64);
    bb &= !create_bb(bit);
}
