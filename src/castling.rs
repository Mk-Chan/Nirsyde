use crate::utils::Printable;

#[derive(Copy, Clone)]
pub struct CastlingRights {
    pub rights: u8
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum CastlingRightType {
    WhiteKingSide = 1, WhiteQueenSide = 2,
    BlackKingSide = 4, BlackQueenSide = 8
}

impl CastlingRights {
    pub fn grant(&mut self, right: CastlingRightType) {
        self.rights |= right as u8;
    }

    pub fn revoke(&mut self, right: CastlingRightType) {
        self.rights &= !(right as u8);
    }
}

impl Printable for CastlingRights {
    fn print(&self) {
        println!("{:>04b}", self.rights);
    }
}
