use std::io;
use std::io::Write;

use backend::board;
use backend::move_gen;
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;

// Test fens: r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - - -

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
                for (from, to, promote) in move_gen::iter_moves(&moves, &board) {
                    print_move(from, to, promote);
                }
            }
            cmd if cmd.starts_with("move ") => {
                let parts: Vec<_> = cmd.split_whitespace().collect();
                if parts.len() != 2 {
                    println!("usage: move <move>");
                    continue;
                }
                let from = parse_squares(&parts[1][0..2]).unwrap();
                let to = parse_squares(&parts[1][2..4]).unwrap();
                let promote = &parts[1][4..];
                board = board.make_move(
                    from,
                    to,
                    match promote {
                        "N" => Some(board::Piece::N),
                        "B" => Some(board::Piece::B),
                        "R" => Some(board::Piece::R),
                        "Q" => Some(board::Piece::Q),
                        _ => None,
                    },
                );
                turn = turn.other();
                println!("{}", board);
            }
            cmd if cmd.starts_with("load ") => {
                let fen = &cmd[5..];
                (board, turn) = board::Board::from_fen(fen).unwrap();
            }
            "perft" => {
                println!("Doing perft");
                for i in 1..10 {
                    let start = std::time::Instant::now();
                    let count = perft(&board, &turn, i);
                    println!("{} -> {}: {:#?}", i, count, start.elapsed());
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
    move_gen::iter_moves(&moves, &board)
        .par_bridge()
        .map(|(from, to, promote)| {
            let next = board.make_move(from, to, promote);
            perft(&next, &turn.other(), depth - 1)
        })
        .sum()
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

fn print_move(from: u64, to: u64, promote: Option<board::Piece>) {
    fn sq_name(sq: u32) -> String {
        let file = (b'a' + (sq % 8) as u8) as char;
        let rank = (b'1' + (sq / 8) as u8) as char;
        format!("{file}{rank}")
    }
    let from = from.trailing_zeros();
    let to = to.trailing_zeros();
    println!(
        "{}{}{}",
        sq_name(from),
        sq_name(to),
        if let Some(p) = promote {
            match p {
                board::Piece::N => "N",
                board::Piece::B => "B",
                board::Piece::Q => "Q",
                board::Piece::R => "R",
                board::Piece::K => "K",
                board::Piece::P => "P",
            }
        } else {
            ""
        }
    );
}

fn print_bitboard(b: &u64) {
    for i in 0..8 {
        println!("{:08b}", b >> ((7 - i) * 8) & 0xff);
    }
}
