use crate::bitboard::Bitboard;

pub const ROOKS_START: Bitboard = Bitboard::new(0b1000000100000000000000000000000000000000000000000000000010000001);

pub fn rook_all_moves(rooks: Bitboard, obsticales: Bitboard) -> Bitboard{
    (rooks.fill(obsticales, Bitboard::north_one)| 
    rooks.fill(obsticales, Bitboard::south_one)| 
    rooks.fill(obsticales, Bitboard::east_one)| 
    rooks.fill(obsticales, Bitboard::west_one)) ^ rooks
}

#[cfg(test)]
mod tests{

    #[test]
    fn rook_all_moves(){
        let mut rook = crate::bitboard::Bitboard::new(0);
        rook.flip_bit(0);
        let mut obsticales = crate::bitboard::Bitboard::new(0);
        let mut moves = super::rook_all_moves(rook, obsticales);
        assert_eq!(moves.popultion_count(), 14);
        obsticales.flip_bit(1);
        moves = super::rook_all_moves(rook, obsticales);
        assert_eq!(moves.popultion_count(), 8);
    }
}