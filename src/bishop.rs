use crate::bitboard::Bitboard;

pub const BISHOP_STARTS: Bitboard = Bitboard::new(0b0010010000000000000000000000000000000000000000000000000000100100);

pub fn bishop_all_moves(bishops: Bitboard, obsticales: Bitboard) -> Bitboard{
    bishops.fill(obsticales, Bitboard::north_east_one) | 
    bishops.fill(obsticales, Bitboard::north_west_one)  | 
    bishops.fill(obsticales, Bitboard::south_east_one)  | 
    bishops.fill(obsticales, Bitboard::south_west_one)
}