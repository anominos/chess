mod move_gen;

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
    println!("Hello, World!");
}
