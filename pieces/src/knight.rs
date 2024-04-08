use crate::bitboard::Bitboard;

pub const KNIGHTS_START: Bitboard = Bitboard::new(0b0100001000000000000000000000000000000000000000000000000001000010);

pub fn knight_all_moves(knights: Bitboard) -> Bitboard{
    knights.north_one().north_west_one() |
    knights.north_one().north_east_one() |
    knights.south_one().south_west_one() |
    knights.south_one().south_east_one() |
    knights.east_one().north_east_one() |
    knights.east_one().south_east_one() |
    knights.west_one().north_west_one() |
    knights.west_one().south_west_one()
}

#[cfg(test)]
mod tests{

    #[test]
    fn knight_all_moves(){
        let mut knight = crate::bitboard::Bitboard::new(0);
        knight.flip_bit(36);
        let mut moves = super::knight_all_moves(knight);
        assert_eq!(moves.popultion_count(), 8);
        knight.flip_bit(36);
        knight.flip_bit(0);
        moves = super::knight_all_moves(knight);
        assert_eq!(moves.popultion_count(), 2);
    }
}