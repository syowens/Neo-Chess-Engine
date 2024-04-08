use std::{fmt::Display, ops::Index};
use pieces::{pawn, knight, bishop, rook, queen, king, bitboard};
mod board;

struct Move(i32, i32);

struct Node{
    uci: String,
    move_type: i32,
    capture: i32,
    board: board::Board,
    next: Vec<Node>
}


fn main() {
    let quit: String = String::from("quit");
    let move_piece: String = String::from("move");
    let mut new_board = board::new_standard();
    let mut uci_moves: Vec<String> = Vec::new();
    let mut run: bool = true;
    let moves = new_board.calculate_moves();
    println!("{}", new_board);
    for mv in moves{
        print!("{} ", move_to_uci((mv.0, mv.1)))
    };
}

fn move_to_uci(mv: (i32, i32)) -> String{
    format!("{}{}", index_to_uci(mv.0),index_to_uci(mv.1))
}

fn uci_to_move(uci: &String) -> (i32, i32){
    let first = &uci[0..2];
    let second = &uci[2..4];
    return (uci_to_index(first), uci_to_index(second))
}

fn index_to_uci(index: i32) -> String{
    let row:i32 = index / 8;
    let column:i32= 7 - (index - (8 * row));
    let mut str: String = String::new();
    str.push(char::from(('a' as i32 + column) as u8));
    str.push_str(&i32::to_string(&(row + 1)));
    return str;
}

fn uci_to_index(uci: &str) -> i32{
    let column = 104 - uci.as_bytes()[0];
    let row:u8 = uci.as_bytes()[1] - 49;
    return (row * 8 + column).into();
}

impl Display for Move{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        writeln!(f, "{}{} ", index_to_uci(self.0), index_to_uci(self.1))
    }
}













