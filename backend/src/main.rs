use backend::board;
use backend::move_gen;

fn main() {
    let board = board::Board::default();
    let turn = board::Colour::W;

    let board = board.make_move(1 << 10, 1 << (10 + 16));
    println!("{}", board);
    println!("Hello, World!");
}

fn print_bitboard(b: &u64) {
    for i in 0..8 {
        println!("{:08b}", b >> ((7 - i) * 8) & 0xff);
    }
}
