use crate::bitboard::Bitboard;


pub const KINGS_START: Bitboard = Bitboard::new(0b100000000000000000000000000000000000000000000000000000001000);

pub fn king_north(king: Bitboard) -> Bitboard{
    king.north_one()
}

pub fn king_north_east(king: Bitboard) -> Bitboard{
    king.north_east_one()
}

pub fn king_north_west(king: Bitboard) -> Bitboard{
    king.north_west_one()
}

pub fn king_east(king: Bitboard) -> Bitboard{
    king.east_one()
}

pub fn king_south(king: Bitboard) -> Bitboard{
    king.south_one()
}

pub fn king_south_east(king: Bitboard) -> Bitboard{
    king.south_east_one()
}

pub fn king_south_west(king: Bitboard) -> Bitboard{
    king.south_west_one()
}

pub fn king_west(king: Bitboard) -> Bitboard{
    king.west_one()
}

pub fn king_all_moves(king: Bitboard) -> Bitboard{
    king_west(king) |
    king_east(king) | 
    king_north(king) | 
    king_north_east(king) |
    king_north_west(king) |
    king_south(king) |
    king_south_east(king) |
    king_south_west(king)
}