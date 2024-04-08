use std::fmt::Display;

use crate::{bishop, bitboard::{self, Bitboard}, king, knight, pawn, queen, rook};

pub struct Board{
    pub pawns: Bitboard,
    pub knights: Bitboard,
    pub bishops: Bitboard,
    pub rooks: Bitboard,
    pub queens: Bitboard,
    pub kings: Bitboard,
    pub player_pieces: Bitboard,
    pub all: Bitboard,
    pub en_passant: Bitboard,
    pub white_king_side_castleble: bool,
    pub black_king_side_castleble: bool,
    pub white_queen_side_castleble: bool,
    pub black_queen_side_castleble: bool,
    pub ply: i32,
    pub turn: i32,
}

impl Board{

    pub fn get_piece_bitboards(&self, square: &Bitboard) -> (i32, i32){
        let check = square & self.all;
        if check.value() == 0{
            return (-1, -1);
        }

        let mut piece = 0;

        if (square & self.kings).value() != 0{
            piece = 5;
        } else if (square & self.queens).value() != 0 {
            piece = 4;
        } else if (square & self.rooks).value() != 0 {
            piece = 3;
        } else if (square & self.bishops).value() != 0 {
            piece = 2;
        } else if (square & self.knights).value() != 0 {
            piece = 1;
        } else if (square & self.pawns).value() != 0 {
            piece = 0;
        }

        let color = if (square & self.player_pieces).value() > 0 {self.ply} else { if self.ply == 0 {1} else {0}};
        return (piece, color);
    }

    pub fn move_piece(&mut self, mve: (i32, i32, i32, i32)) -> Board{
        let mut new_board = self.clone();
        new_board.en_passant = Bitboard::new(0);
        let start = Bitboard::new(1 << mve.0);
        let end = Bitboard::new(1 << mve.1);
        if mve.2 != 11{
            match mve.3 {
                0 => new_board.pawns = if mve.2 == 7 {new_board.pawns ^ pawn::pawn_single_push(end, if self.ply == 0 {0} else {1})} else {new_board.pawns ^ end},
                1 => new_board.knights = new_board.knights ^ end,
                2 => new_board.bishops = new_board.bishops ^ end,
                3 => {
                    new_board.rooks = new_board.rooks ^ end;
                    match mve.1{
                        0 => new_board.white_king_side_castleble = false,
                        7 => new_board.white_queen_side_castleble = false,
                        56 => new_board.black_king_side_castleble = false,
                        63 => new_board.black_queen_side_castleble = false,
                        _ => ()
                    }
                }
                4 => new_board.queens = new_board.queens ^ end,
                5 => new_board.kings = new_board.kings ^ end,
                _ => ()
            }
        }
        match mve.2{
            0 | 1 | 2 => {
                new_board.pawns = (new_board.pawns ^ start) | end;
                new_board.player_pieces = (new_board.player_pieces ^ start) | end;
                if mve.2 == 1{
                    new_board.en_passant = end.clone();
                }
            }
            3 => {
                new_board.knights = (new_board.knights ^ start) | end;
                new_board.player_pieces = (new_board.player_pieces ^ start) | end;
            }
            4 => {
                new_board.bishops = (new_board.bishops ^ start) | end;
                new_board.player_pieces = (new_board.player_pieces ^ start) | end;
            }
            5 => {
                match mve.0{
                    0 => new_board.white_king_side_castleble = false,
                    7 => new_board.white_queen_side_castleble = false,
                    56 => new_board.black_king_side_castleble = false,
                    63 => new_board.black_queen_side_castleble = false,
                    _ => ()
                }
                new_board.rooks = (new_board.rooks ^ start) | end;
                new_board.player_pieces = (new_board.player_pieces ^ start) | end;


            }
            6 => {
                new_board.queens = (new_board.queens ^ start) | end;
                new_board.player_pieces = (new_board.player_pieces ^ start) | end;

            }
            7 => {
                new_board.kings = (new_board.kings ^ start) | end;
                new_board.player_pieces = (new_board.player_pieces ^ start) | end;
                if self.ply == 0 {
                    new_board.white_king_side_castleble = false;
                    new_board.white_queen_side_castleble = false;
                } else {
                    new_board.black_king_side_castleble = false;
                    new_board.black_queen_side_castleble = false;
                }
            }
            8 => {
                new_board.pawns = (new_board.pawns ^ start) ^ end;
                new_board.player_pieces = (new_board.player_pieces ^ start) | if self.ply == 0 {end.north_one()} else {end.south_one()}
            }
            9 => {
                new_board.kings = (new_board.kings ^ start) | end.west_one();
                new_board.rooks = (new_board.rooks ^ end) | start.east_one();
                new_board.player_pieces = ((new_board.player_pieces ^ start) ^ end) | end.west_one() | start.east_one();
                if self.ply == 0 {
                    new_board.white_king_side_castleble = false;
                    new_board.white_queen_side_castleble = false;
                } else {
                    new_board.black_king_side_castleble = false;
                    new_board.black_queen_side_castleble = false;
                }
            }
            10 => {
                new_board.kings = (new_board.kings ^ start) | end.east_one().east_one();
                new_board.rooks = (new_board.rooks ^ end) | start.west_one();
                new_board.player_pieces = ((new_board.player_pieces ^ start) ^ end) | end.east_one().east_one() | start.west_one();
                if self.ply == 0 {
                    new_board.white_king_side_castleble = false;
                    new_board.white_queen_side_castleble = false;
                } else {
                    new_board.black_king_side_castleble = false;
                    new_board.black_queen_side_castleble = false;
                }
            }
            11 => {
                new_board.pawns = new_board.pawns ^ start;
                new_board.player_pieces = (new_board.player_pieces ^ start) ^ end;
                match mve.3{
                    1 => new_board.knights = new_board.knights ^ end,
                    2 => new_board.bishops = new_board.bishops ^ end,
                    3 => new_board.rooks = new_board.rooks ^ end,
                    4 => new_board.queens = new_board.queens ^ end,
                    _=> ()
                }

            }
            _ => ()
        }
        new_board.all = new_board.pawns | new_board.knights | new_board.bishops | new_board.rooks | new_board.queens | new_board.kings;
        new_board.player_pieces = new_board.all ^ new_board.player_pieces;
        new_board.ply = if new_board.ply == 0 {1} else {0};
        new_board.turn = if new_board.ply == 0 {new_board.turn + 1} else {new_board.turn};
        return new_board;

    }

    pub fn num_player_attacking_square(&self, square: Bitboard) -> u32{
        let pawn_attackers = pawn::get_all_pawn_attackers(square, self.player_pieces & self.pawns, self.ply);
        let knight_attackers = knight::knight_all_moves(square) & (self.knights & self.player_pieces);
        let bishop_attackers =  bishop::bishop_all_moves(square, self.all) & (self.bishops & self.player_pieces);
        let rook_attackers = rook::rook_all_moves(square, self.all) & (self.rooks & self.player_pieces);
        let queen_attackers = queen::queen_all_moves(square, self.all) & (self.queens & self.player_pieces);
        let king_attackers = king::king_all_moves(square) & (self.kings & self.player_pieces);
        return pawn_attackers.popultion_count() + knight_attackers.popultion_count() + bishop_attackers.popultion_count() + rook_attackers.popultion_count() + queen_attackers.popultion_count() + king_attackers.popultion_count();
    }

    pub fn num_opponent_attacking_square(&self, square: Bitboard) -> u32{
        let pawn_attackers = pawn::get_all_pawn_attackers(square, -self.player_pieces & self.pawns, if self.ply == 1 {0} else {1});
        let knight_attackers = knight::knight_all_moves(square) & (self.knights & -self.player_pieces);
        let bishop_attackers =  bishop::bishop_all_moves(square, self.all) & (self.bishops & -self.player_pieces);
        let rook_attackers = rook::rook_all_moves(square, self.all) & (self.rooks & -self.player_pieces);
        let queen_attackers = queen::queen_all_moves(square, self.all) & (self.queens & -self.player_pieces);
        let king_attackers = king::king_all_moves(square) & (self.kings & -self.player_pieces);
        return pawn_attackers.popultion_count() + knight_attackers.popultion_count() + bishop_attackers.popultion_count() + rook_attackers.popultion_count() + queen_attackers.popultion_count() + king_attackers.popultion_count();
    }

    pub fn calculate_moves(&mut self) -> Vec<(i32, i32, i32, i32, Board)> {
        let mut canditates: Vec<(i32, i32, i32, i32, Board)> = Vec::new();
        let mut pawn_single_push = pawn::pawn_single_push(self.player_pieces & self.pawns, self.ply) & -self.all;
        let mut pawn_double_push = pawn::pawn_double_push(pawn_single_push, self.ply) & -self.all;
        let mut pawn_attacks = pawn::pawn_all_attacks(self.pawns & self.player_pieces, self.ply) & (self.all ^ self.player_pieces);
        let mut knight_moves= knight::knight_all_moves(self.knights & self.player_pieces) & -self.player_pieces;
        let mut bishop_moves = bishop::bishop_all_moves(self.bishops & self.player_pieces, self.all) & -self.player_pieces;
        let mut rook_moves = rook::rook_all_moves(self.rooks & self.player_pieces, self.all) & -self.player_pieces; 
        let mut queen_moves = queen::queen_all_moves(self.queens & self.player_pieces, self.all) & -self.player_pieces;
        let mut king_moves = king::king_all_moves(self.kings & self.player_pieces) & -self.player_pieces;

        while pawn_single_push.value() != 0{
            let location = pawn_single_push.least_signicant();
            let attacker = (location.north_one() & self.player_pieces) | (location.south_one() & self.player_pieces);
            if ((location & bitboard::EIGHT_RANK) | (location & bitboard::FIRST_RANK)).value() > 0{
                let mut piece = 1;
                while piece < 5 {
                    let new_board = self.move_piece((attacker.least_signicant_index(), location.least_signicant_index(), 11, piece));
                    if new_board.num_player_attacking_square(self.kings & self.player_pieces) == 0{
                        canditates.push((attacker.least_signicant_index(), location.least_signicant_index(), 11, piece, new_board));
                    }
                    else{
                        break;
                    }
                    piece = piece + 1;
                }
            }
            let new_board = self.move_piece((attacker.least_signicant_index(), location.least_signicant_index(), 0, -1));
            if new_board.num_player_attacking_square(self.kings & self.player_pieces) == 0{
                canditates.push((attacker.least_signicant_index(), location.least_signicant_index(), 0, -1, new_board));
            }
            pawn_single_push = pawn_single_push ^ location
        }
    
        while pawn_double_push.value() != 0{
            let location = pawn_double_push.least_signicant();
            let attacker = (location.north_one().north_one() & self.player_pieces) | (location.south_one().south_one() & self.player_pieces);
            let new_board = self.move_piece((attacker.least_signicant_index(), location.least_signicant_index(), 1, -1));
            if new_board.num_player_attacking_square(self.kings & self.player_pieces) == 0{
                canditates.push((attacker.least_signicant_index(), location.least_signicant_index(), 1, -1, new_board));
            }
            pawn_double_push = pawn_double_push ^ location
        }

        while pawn_attacks.value() != 0{
            let location = pawn_attacks.least_signicant();
            let mut attackers = (location.north_east_one() | location.north_west_one() | location.south_east_one() | location.south_west_one()) & self.pawns & self.player_pieces;
            while attackers.value() != 0{
                let attacker = attackers.least_signicant();
                let new_board = self.move_piece((attacker.least_signicant_index(), location.least_signicant_index(), 2, -1));
                if new_board.num_player_attacking_square(self.kings & self.player_pieces) == 0{
                    let captue = self.get_piece_bitboards(&location).0;
                    canditates.push((attacker.least_signicant_index(), location.least_signicant_index(), 2, captue, new_board));
                }
                attackers = attackers ^ attacker;
            }
            pawn_attacks = pawn_attacks ^ location
        }
    
        while knight_moves.value() != 0{
            let location = knight_moves.least_signicant();
            let mut attackers = knight::knight_all_moves(location) & (self.knights & self.player_pieces);
            while attackers.value() != 0{
                let attacker = attackers.least_signicant();
                let new_board = self.move_piece((attacker.least_signicant_index(), location.least_signicant_index(), 3, self.get_piece_bitboards(&location).0));
                if new_board.num_player_attacking_square(self.kings & self.player_pieces) == 0{
                    canditates.push((attacker.least_signicant_index(), location.least_signicant_index(), 3, self.get_piece_bitboards(&location).0, new_board));
                }
                attackers = attackers ^ attackers.least_signicant();
            }
            knight_moves = knight_moves ^ location;
        }
    
        while bishop_moves.value() != 0{
            let location = bishop_moves.least_signicant();
            let mut attackers = bishop::bishop_all_moves(location, self.all) & (self.bishops & self.player_pieces);
            while attackers.value() != 0{
                let attacker = attackers.least_signicant();
                let new_board = self.move_piece((attacker.least_signicant_index(), location.least_signicant_index(), 4, self.get_piece_bitboards(&location).0));
                if new_board.num_player_attacking_square(self.kings & self.player_pieces) == 0{
                    canditates.push((attacker.least_signicant_index(), location.least_signicant_index(), 4, self.get_piece_bitboards(&location).0, new_board));
                }
                attackers = attackers ^ attackers.least_signicant();
            }
            bishop_moves = bishop_moves ^ location;
        }
    
        while rook_moves.value() != 0{
            let location = rook_moves.least_signicant();
            let mut attackers = rook::rook_all_moves(location, self.all) & (self.rooks & self.player_pieces);
            while attackers.value() != 0{
                let attacker = attackers.least_signicant();
                let new_board = self.move_piece((attacker.least_signicant_index(), location.least_signicant_index(), 5, self.get_piece_bitboards(&location).0));
                if new_board.num_player_attacking_square(self.kings & self.player_pieces) == 0{
                    canditates.push((attacker.least_signicant_index(), location.least_signicant_index(), 5, self.get_piece_bitboards(&location).0, new_board));
                }
                attackers = attackers ^ attackers.least_signicant();
            }
            rook_moves = rook_moves ^ location;
        }
    
        while queen_moves.value() != 0{
            let location = queen_moves.least_signicant();
            let mut attackers = queen::queen_all_moves(location, self.all) & (self.queens & self.player_pieces);
            while attackers.value() != 0{
                let attacker = attackers.least_signicant();
                let new_board = self.move_piece((attacker.least_signicant_index(), location.least_signicant_index(), 6, self.get_piece_bitboards(&location).0));
                if new_board.num_player_attacking_square(self.kings & self.player_pieces) == 0{
                    canditates.push((attacker.least_signicant_index(), location.least_signicant_index(), 6, self.get_piece_bitboards(&location).0, new_board));
                }
                attackers = attackers ^ attackers.least_signicant();
            }
            queen_moves = queen_moves ^ location;
        }

    
        while king_moves.value() != 0{
            let location = king_moves.least_signicant();
            let capture = self.get_piece_bitboards(&location).0;
            let attacker = self.kings & self.player_pieces;
            let new_board = self.move_piece((attacker.least_signicant_index(), location.least_signicant_index(), 7, capture));
            if new_board.num_player_attacking_square(self.kings & self.player_pieces) == 0{
                canditates.push((attacker.least_signicant_index(), location.least_signicant_index(), 7, capture, new_board));
            }
            king_moves = king_moves ^ location
        }

        if self.en_passant.value() > 0{
            let mut attackers = (self.en_passant.west_one() & self.pawns) | (self.en_passant.east_one() & self.pawns);
            while attackers.value() != 0 {
                let attacker = attackers.least_signicant();
                let new_board = self.move_piece((attacker.least_signicant_index(), self.en_passant.least_signicant_index(), 8, 0));
                if new_board.num_player_attacking_square(self.kings & self.player_pieces) == 0{
                    canditates.push((attacker.least_signicant_index(), self.en_passant.least_signicant_index(), 8, 1, new_board));
                }
                attackers = attackers ^ attacker;
            }
        }

        let king_castle = if self.ply == 0 {self.white_king_side_castleble} else {self.black_king_side_castleble};
        let queen_castle = if self.ply == 0 {self.white_queen_side_castleble} else {self.black_queen_side_castleble};

        if king_castle{
            let mut empty_squares = if self.ply == 0 {Bitboard::new(3 << 1)} else {Bitboard::new(3 << 57)} & -self.all;
            if empty_squares.popultion_count() == 2{
                let mut all_safe = true;
                while (empty_squares.value() != 0) & all_safe{
                    println!("{}", empty_squares);
                    all_safe = self.num_opponent_attacking_square(empty_squares.least_signicant()) == 0;
                    empty_squares = empty_squares ^ empty_squares.least_signicant();
                }
                if all_safe{
                    let attacker = self.player_pieces & self.kings;
                    let location = if self.ply == 0 {Bitboard::new(0)} else {Bitboard::new(1<<56)};
                    let new_board = self.move_piece((attacker.least_signicant_index(), location.least_signicant_index(), 9, -1));
                    canditates.push((attacker.least_signicant_index(), location.least_signicant_index(), 9, -1, new_board));
                }
            }
        }

        if queen_castle{
            let mut empty_squares = if self.ply == 0 {Bitboard::new(7 << 4)} else {Bitboard::new(7 << 60)}& -self.all;
            if empty_squares.popultion_count() == 3{
                let mut all_safe = true;
                while (empty_squares.value() != 0) & all_safe{
                    all_safe = self.num_player_attacking_square(empty_squares.least_signicant()) == 0;
                    empty_squares = empty_squares ^ empty_squares.least_signicant();
                }
                if all_safe{
                    let attacker = self.player_pieces & self.kings;
                    let location = if self.ply == 0 {Bitboard::new(1<<7)} else {Bitboard::new(1<<63)};
                    let new_board = self.move_piece((attacker.least_signicant_index(), location.least_signicant_index(), 10, -1));
                    canditates.push((attacker.least_signicant_index(), location.least_signicant_index(), 10, -1, new_board));
                }

            }
        }

        return canditates;
    
    }


}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        let mut i: u64 = 64;
        while i >= 1{
            let piece = self.get_piece_bitboards(&Bitboard::new(1 << i-1));
            match piece.0{
                0 => if piece.1 == 0 {str.push('p')} else {str.push('P')},
                1 => if piece.1 == 0 {str.push('n')} else {str.push('N')},
                2 => if piece.1 == 0 {str.push('b')} else {str.push('B')},
                3 => if piece.1 == 0 {str.push('r')} else {str.push('R')},
                4 => if piece.1 == 0 {str.push('q')} else {str.push('Q')},
                5 => if piece.1 == 0 {str.push('k')} else {str.push('K')},
                _ => str.push('.'),
            }
            if (i-1) % 8 == 0{
                str.push('\n');
            } else {
                str.push(' ');
            }
            i = i-1;
        }
        return write!(f, "{}", str);
    }
}
impl PartialEq for Board{
    fn eq(&self, rhs: &Self) -> bool {
        (self.pawns == rhs.pawns) &
        (self.knights == rhs.knights) &
        (self.bishops == rhs.bishops) &
        (self.rooks == rhs.rooks) &
        (self.queens == rhs.queens) &
        (self.kings == rhs.kings) &
        (self.player_pieces == rhs.player_pieces) 
    }
}

impl Clone for Board{

    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for Board{
}



pub fn new_standard() -> Board{
    Board{
        pawns: pawn::PAWNS_START,
        knights: knight::KNIGHTS_START,
        bishops: bishop::BISHOP_STARTS,
        rooks: rook::ROOKS_START,
        queens: queen::QUEENS_START,
        kings: king::KINGS_START,
        player_pieces: Bitboard::new((1 << 16) - 1),
        all: pawn::PAWNS_START | knight::KNIGHTS_START | bishop::BISHOP_STARTS | rook::ROOKS_START | queen::QUEENS_START | king::KINGS_START,
        en_passant: Bitboard::new(0),
        white_king_side_castleble: true,
        black_king_side_castleble: true,
        white_queen_side_castleble: true,
        black_queen_side_castleble: true,
        ply: 0,
        turn: 1,    

    }
}