use crate::bitboard::{Bitboard, SECOND_RANK, SEVENTH_RANK};

pub const PAWNS_START: Bitboard =
    Bitboard::new(0b11111111000000000000000000000000000000001111111100000000);

pub fn pawn_single_push(pawns: Bitboard, color: i32) -> Bitboard {
    match color {
        0 => return pawns.north_one(),
        1 => return pawns.south_one(),
        _ => return Bitboard::new(0),
    }
}

pub fn pawn_double_push(pawns: Bitboard, color: i32) -> Bitboard {
    match color {
        0 => return (pawns & SECOND_RANK).north_one().north_one(),
        1 => return (pawns & SEVENTH_RANK).north_one().south_one(),
        _ => return Bitboard::new(0),
    }
}

pub fn pawn_all_attacks(pawns: Bitboard, color: i32) -> Bitboard {
    match color {
        0 => return pawns.north_west_one() | pawns.north_east_one(),
        1 => return pawns.south_west_one() | pawns.south_east_one(),
        _ => return Bitboard::new(0),
    }
}

pub fn get_all_pawn_attackers(square: Bitboard, pawns: Bitboard, color: i32) -> Bitboard {
    match color {
        0 => return (square.north_east_one() & square.north_west_one()) & pawns,
        1 => return (square.south_east_one() & square.south_west_one()) & pawns,
        _ => return Bitboard::new(0),
    }
}