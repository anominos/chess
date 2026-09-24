use backend::board;
use backend::move_gen;

fn main() {
    let board = board::Board {
        b: [0, 0, 0, 0, 0, 0],
        w: [0, 1 << 41, 1 << 27, 1 << 3, 0, 0],
    };
    let turn = board::Colour::W;

    let moves = move_gen::gen_pseudolegal_moves(board, turn);
    for sq in [27, 41, 3] {
        print_bitboard(&(1 << sq));
        println!();
        print_bitboard(&moves[sq]);
        println!();
        println!();
    }
    println!("Hello, World!");
}

fn print_bitboard(b: &u64) {
    for i in 0..8 {
        println!("{:08b}", b >> ((7 - i) * 8) & 0xff);
    }
}
