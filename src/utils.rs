use crate::types::Bitboard;

pub trait Printable {
    fn print(&self);
}

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
