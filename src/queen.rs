use crate::bitboard::Bitboard;

pub const QUEENS_START: Bitboard = Bitboard::new(0b0001000000000000000000000000000000000000000000000000000000010000);

pub fn queen_all_moves(queens: Bitboard, obstiacles: Bitboard) -> Bitboard{
   queens.fill(obstiacles, Bitboard::north_one) |
   queens.fill(obstiacles, Bitboard::north_east_one) |
   queens.fill(obstiacles, Bitboard::north_west_one) |
   queens.fill(obstiacles, Bitboard::south_one) |
   queens.fill(obstiacles, Bitboard::south_east_one) |
   queens.fill(obstiacles, Bitboard::south_west_one) |
   queens.fill(obstiacles, Bitboard::east_one) |
   queens.fill(obstiacles, Bitboard::west_one)
}