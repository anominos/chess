mod board;
mod move_gen;

mod consts {
    include!(concat!(env!("OUT_DIR"), "/generated.rs"));
}

fn print_bitboard(board: u64) {
    for row in (0..8).rev() {
        for col in 0..8 {
            let bit = 1u64 << (row * 8 + col);
            print!("{} ", if board & bit != 0 { "1" } else { "." });
        }
        println!();
    }
}

fn main() {
    let knight: u64 = 1 << 59;
    let moves = move_gen::get_knight_moves(knight);
    print_bitboard(knight);
    println!();
    print_bitboard(moves);
}
