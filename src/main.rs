use std::{fmt::Display, ops::Index};

use board::Board;

mod bitboard;
mod pawn;
mod knight;
mod king;
mod bishop;
mod queen;
mod rook;
mod board;

struct Move(i32, i32);

struct Node{
    uci: String,
    move_type: i32,
    capture: i32,
    board: Board,
    next: Vec<Node>
}


fn main() {
    let quit: String = String::from("quit");
    let move_piece: String = String::from("move");
    let mut new_board = board::new_standard();
    let mut uci_moves: Vec<String> = Vec::new();
    let mut run: bool = true;

    while run {
        println!{"{}{} {} {} {} {}\n", new_board, new_board.white_king_side_castleble, new_board.white_queen_side_castleble, new_board.black_king_side_castleble, new_board.black_queen_side_castleble,  new_board.en_passant.least_signicant_index()}
        let moves = new_board.calculate_moves();
        for mve in moves.iter(){
            uci_moves.push(move_to_uci((mve.0, mve.1)))
        }
        for uci in uci_moves.iter(){
            print!{"{} ", uci};
        }
        println!();

        let mut line = String::new();
        let mut update: bool = false;
        while !update{
            line = String::new();
            let b1 = std::io::stdin().read_line(&mut line).unwrap();
            let input = line.trim_end().to_string().split_ascii_whitespace().nth(0).unwrap().to_string();
            if input == quit {
                run = false;
                break;
            }

            if input == move_piece{
                let uci = &line.trim_end().to_string().split_ascii_whitespace().nth(1).unwrap().to_string();
                let mut count: i32 = 0;
                if uci_moves.contains(uci){
                    for canditate in uci_moves.iter(){
                        if canditate == uci{
                            break;
                        }
                        count = count + 1;
                    }
                    new_board = moves.iter().nth(count.try_into().unwrap()).unwrap().4;
                    update = true;
                }

            }
        }
        // let mut iter = moves.iter();
        // let chosen = iter.nth(index.into()).unwrap();
        // new_board = new_board.move_piece(*chosen);
        // println!("{:?}", chosen);
        uci_moves = Vec::new();

    }

    // let mut tree = Node{
    //     uci: String::new(),
    //     move_type: -1,
    //     capture: 0,
    //     board: new_board,
    //     next: Vec::new()
    // };
    // let moves = tree.board.calculate_moves();
    // for m in moves.iter(){
    //     let b = tree.board.move_piece(*m);
    //     tree.next.push(Node{uci: move_to_uci((m.0, m.1)), move_type: m.2 , capture: m.3 , board: b, next: Vec::new()})
    // }

    // for b in tree.next{
    //     println!("{} {} {}", b.uci, b.move_type, b.capture);
    // }
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













