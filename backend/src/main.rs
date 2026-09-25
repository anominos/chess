use std::io;
use std::io::Write;

use backend::board;
use backend::move_gen;

fn main() {
    let mut board = board::Board::default();
    let mut turn = board::Colour::W;
    println!("{}", board);
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read user input");
        match input.trim() {
            "quit" | "exit" => break,
            "show" => println!("{}", board),
            "moves" => {
                let moves = move_gen::gen_moves(&board, &turn);
                for (from, to) in move_gen::iter_moves(&moves) {
                    print_move(from, to);
                }
            }
            cmd if cmd.starts_with("move ") => {
                let parts: Vec<_> = cmd.split_whitespace().collect();
                if parts.len() != 3 {
                    println!("usage: move <from> <to>");
                    continue;
                }
                let from = parse_squares(parts[1]).unwrap();
                let to = parse_squares(parts[2]).unwrap();
                board = board.make_move(from, to);
                turn = turn.other();
                println!("{}", board);
            }
            "perft" => {
                println!("Doing perft");
                for i in 1..10 {
                    let count = perft(&board, &turn, i);
                    println!("{} -> {}", i, count);
                }
            }
            _ => println!("invalid command"),
        }
    }
}

fn perft(board: &board::Board, turn: &board::Colour, depth: u32) -> u64 {
    if depth == 0 {
        return 1;
    }
    let moves = move_gen::gen_moves(&board, &turn);
    let mut count = 0;
    for (from, to) in move_gen::iter_moves(&moves) {
        let next = board.make_move(from, to);
        count += perft(&next, &turn.other(), depth - 1);
    }
    count
}

fn parse_squares(s: &str) -> Option<u64> {
    if s.len() != 2 {
        return None;
    }
    let bytes = s.as_bytes();
    let col = match bytes[0] {
        b'a'..=b'h' => bytes[0] - b'a',
        _ => return None,
    };
    let row = match bytes[1] {
        b'1'..=b'8' => bytes[1] - b'1',
        _ => return None,
    };
    Some(1u64 << (row * 8 + col))
}

fn print_move(from: u64, to: u64) {
    fn sq_name(sq: u32) -> String {
        let file = (b'a' + (sq % 8) as u8) as char;
        let rank = (b'1' + (sq / 8) as u8) as char;
        format!("{file}{rank}")
    }
    let from = from.trailing_zeros();
    let to = to.trailing_zeros();
    println!("{}{}", sq_name(from), sq_name(to));
}

fn print_bitboard(b: &u64) {
    for i in 0..8 {
        println!("{:08b}", b >> ((7 - i) * 8) & 0xff);
    }
}
