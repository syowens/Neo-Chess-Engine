use crate::bitboard::Bitboard;

pub const ROOKS_START: Bitboard = Bitboard::new(0b1000000100000000000000000000000000000000000000000000000010000001);

pub fn rook_all_moves(rooks: Bitboard, obsticales: Bitboard) -> Bitboard{
    rooks.fill(obsticales, Bitboard::north_one)| 
    rooks.fill(obsticales, Bitboard::south_one)| 
    rooks.fill(obsticales, Bitboard::east_one)| 
    rooks.fill(obsticales, Bitboard::west_one)
}