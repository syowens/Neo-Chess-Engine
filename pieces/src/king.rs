use crate::bitboard::Bitboard;


pub const KINGS_START: Bitboard = Bitboard::new(0b100000000000000000000000000000000000000000000000000000001000);
pub fn king_all_moves(king: Bitboard) -> Bitboard{
    king.north_one() |
    king.south_one() | 
    king.west_one() | 
    king.east_one() |
    king.north_west_one() |
    king.north_east_one() |
    king.south_west_one() |
    king.south_east_one()
}

#[cfg(test)]
mod tests{

    #[test]
    fn king_all_moves(){
        let mut king = crate::bitboard::Bitboard::new(0);
        king.flip_bit(36);
        let moves = super::king_all_moves(king);
        assert_eq!(moves.popultion_count(), 8);
        king.flip_bit(36);
        king.flip_bit(0);
        let moves = super::king_all_moves(king);
        assert_eq!(moves.popultion_count(), 3);
    }
}