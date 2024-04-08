use crate::bitboard::Bitboard;

pub const KNIGHTS_START: Bitboard = Bitboard::new(0b0100001000000000000000000000000000000000000000000000000001000010);

pub fn knight_north_north_east_target(knights: Bitboard) -> Bitboard{
    return knights.north_one().north_east_one()
}

pub fn knight_north_north_west_target(knights: Bitboard) -> Bitboard{
    return knights.north_one().north_west_one();
}

pub fn knight_west_west_north_target(knights: Bitboard) -> Bitboard{
    return knights.west_one().north_west_one()
}

pub fn knight_west_west_south_target(knights: Bitboard) -> Bitboard{
    return knights.west_one().south_west_one()
}

pub fn knight_south_south_west_target(knights: Bitboard) -> Bitboard{
    return knights.south_one().south_west_one()
}

pub fn knight_south_south_east_target(knights: Bitboard) -> Bitboard{
    return knights.south_one().south_east_one()
}

pub fn knight_east_east_south_target(knights: Bitboard) -> Bitboard{
    return knights.east_one().south_east_one()
}

pub fn knight_east_east_north_target(knights: Bitboard) -> Bitboard{
    return knights.east_one().north_east_one()
}

pub fn knight_all_targets(knights: Bitboard) -> Bitboard{
    return knight_east_east_north_target(knights) |
    knight_east_east_south_target(knights)  |
    knight_north_north_east_target(knights) |
    knight_north_north_west_target(knights) |
    knight_south_south_east_target(knights) |
    knight_south_south_west_target(knights) |
    knight_west_west_north_target(knights)  |
    knight_west_west_south_target(knights);
}