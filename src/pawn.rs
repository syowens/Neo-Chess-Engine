use crate::bitboard::{Bitboard, SECOND_RANK, SEVENTH_RANK};

pub const PAWNS_START: Bitboard = Bitboard::new(0b11111111000000000000000000000000000000001111111100000000);

    pub fn en_passant(target: Bitboard, pawns: Bitboard, color: i32) -> Bitboard{
        let mut attackers = pawns & if color == 0 {SEVENTH_RANK.north_one().north_one()} else {SECOND_RANK.south_one().south_one()};
        if attackers.value() != 0{
            attackers = (target.east_one() & attackers) | (target.west_one() & attackers);
        }
        return attackers;
    }

    pub fn pawn_single_push(pawns: Bitboard,color: i32) -> Bitboard{
        match color {
            0 => return pawns.north_one(),
            1 => return pawns.south_one(),
            _ => return Bitboard::new(0)
        }
    }

    pub fn pawn_double_push(pawns: Bitboard, color: i32) -> Bitboard{
        match color {
            0 => return (pawns & SECOND_RANK << 8).north_one(),
            1 => return (pawns & SEVENTH_RANK >> 8).south_one(),
            _ => return Bitboard::new(0)
        }
    }

    pub fn pawn_east_attack(pawns: Bitboard, color: i32) -> Bitboard{
        match color {
            0 => return pawns.north_east_one(),
            1 => return pawns.south_east_one(),
            _ => return Bitboard::new(0)
        }        
    }

    pub fn pawn_west_attack(pawns: Bitboard, color: i32) -> Bitboard{
        match color {
            0 => return pawns.north_west_one(),
            1 => return pawns.south_west_one(),
            _ => return Bitboard::new(0)
        }
    }

    pub fn pawn_all_attacks(pawns: Bitboard, color: i32) -> Bitboard{
        match color {
            0 => return white_pawns_west_attacks(pawns) | white_pawns_east_attacks(pawns),
            1 => return black_pawns_west_attacks(pawns) | black_pawns_east_attacks(pawns),
            _ => return Bitboard::new(0)
        }
    }

    pub fn get_all_pawn_attackers(square: Bitboard, pawns: Bitboard, color: i32) -> Bitboard{
        match color {
            0 => return (square.north_east_one() & square.north_west_one()) & pawns,
            1 => return (square.south_east_one() & square.south_west_one()) & pawns,
            _=> return Bitboard::new(0)
        }
    }

    pub fn white_single_push_target(pawns: Bitboard, all: Bitboard) -> Bitboard{
        return pawns.north_one() & -all;
    }

    pub fn white_double_push_target(pawns: Bitboard, all: Bitboard) -> Bitboard{
        return ((pawns & SECOND_RANK).north_one() & -all).north_one() & -all;
    }

    pub fn black_single_push_target(pawns: Bitboard, all: Bitboard) -> Bitboard{
        return pawns.south_one() & -all;
    }

    pub fn black_double_push_target(pawns: Bitboard, all: Bitboard) -> Bitboard{
        return ((pawns & SEVENTH_RANK).south_one() & -all).south_one() & -all;
    }

    pub fn white_pawns_able_single_push(pawns: Bitboard, all: Bitboard) -> Bitboard{
        return -all.south_one() & pawns;
    }

    pub fn white_pawns_able_double_push(pawns: Bitboard, all: Bitboard) -> Bitboard{
        return -all.south_one().south_one() & white_pawns_able_single_push(pawns, all) & SECOND_RANK;
    }

    pub fn black_pawns_able_single_push(pawns: Bitboard, all: Bitboard) -> Bitboard{
        return -all.north_one() & pawns;
    }

    pub fn black_pawns_able_double_push(pawns: Bitboard, all: Bitboard) -> Bitboard{
        return -all.north_one().north_one() & black_pawns_able_single_push(pawns, all) & SEVENTH_RANK;
    }

    pub fn white_pawns_east_attacks(pawns: Bitboard) -> Bitboard{
        return pawns.north_east_one();
    }

    pub fn white_pawns_west_attacks(pawns: Bitboard) -> Bitboard{
        return pawns.north_west_one();
    }

    pub fn black_pawns_east_attacks(pawns: Bitboard) -> Bitboard{
        return pawns.south_east_one();
    }

    pub fn black_pawns_west_attacks(pawns: Bitboard) -> Bitboard{
        return pawns.south_west_one();
    }

    pub fn pawn_any_attacks(east_attacks: Bitboard, west_attacks: Bitboard) -> Bitboard{
        return east_attacks | west_attacks;
    }

    pub fn pawn_double_attacks(east_attacks: Bitboard, west_attacks: Bitboard) -> Bitboard{
        return east_attacks & west_attacks;
    }

    pub fn pawn_single_attacks(east_attacks: Bitboard, west_attacks: Bitboard) -> Bitboard{
        return east_attacks ^ west_attacks;
    }

    pub fn white_pawns_able_to_capture_east(pawns: Bitboard, black_pieces: Bitboard) -> Bitboard {
        return black_pieces.south_west_one() & pawns;
    }

    pub fn white_pawns_able_to_capture_west(pawns: Bitboard, black_pieces: Bitboard) -> Bitboard {
        return black_pieces.south_east_one() & pawns;
    }

    pub fn white_pawns_able_to_capture_any(pawns: Bitboard, black_pieces: Bitboard) -> Bitboard {
        return (black_pieces.south_east_one() | black_pieces.south_west_one()) & pawns;
    }

    pub fn black_pawns_able_to_capture_east(pawns: Bitboard, white_pieces: Bitboard) -> Bitboard {
        return white_pieces.north_west_one() & pawns;
    }

    pub fn black_pawns_able_to_capture_west(pawns: Bitboard, white_pieces: Bitboard) -> Bitboard {
        return white_pieces.north_east_one() & pawns;
    }

    pub fn black_pawns_able_to_capture_any(pawns: Bitboard, white_pieces: Bitboard) -> Bitboard {
        return (white_pieces.north_east_one() | white_pieces.north_west_one()) & pawns;
    }

    pub fn white_pawm_rams(white_pawns: Bitboard, black_pawns: Bitboard) -> Bitboard{
        return black_pawns.south_one() & white_pawns;
    }

    pub fn black_pawm_rams(white_pawns: Bitboard, black_pawns: Bitboard) -> Bitboard{
        return black_pawns & white_pawns.north_one();
    }