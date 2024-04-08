use crate::bitboard::Bitboard;

pub const BISHOP_STARTS: Bitboard = Bitboard::new(0b0010010000000000000000000000000000000000000000000000000000100100);

pub fn bishop_all_moves(bishops: Bitboard, obsticales: Bitboard) -> Bitboard{
    (bishops.fill(obsticales, Bitboard::north_east_one) | 
    bishops.fill(obsticales, Bitboard::north_west_one)  | 
    bishops.fill(obsticales, Bitboard::south_east_one)  | 
    bishops.fill(obsticales, Bitboard::south_west_one)) ^ bishops
}

#[cfg(test)]
mod tests{
    

    


    #[test]
    fn bishop_all_moves(){
        let mut bishops = crate::bitboard::Bitboard::new(0);
        bishops.flip_bit(36);
        let mut obsticales = crate::bitboard::Bitboard::new(0);
        let mut moves = super::bishop_all_moves(bishops, obsticales);
        println!("{}", moves);
        assert_eq!(moves.popultion_count(), 13);
        obsticales.flip_bit(43);
        moves = super::bishop_all_moves(bishops, obsticales);
        println!("{}", moves);
        assert_eq!(moves.popultion_count(), 11);
    }
}