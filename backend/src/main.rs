mod board;

fn main() {
    let b = board::Board::default();
    b.validate();
    println!("{}", b);
}
