#[path = "../generated/magics.rs"]
mod magics;
use magics::{BISHOP_MAGICS, ROOK_MAGICS};

fn main() {
    println!("Rook size: {:.2}kB", calc_size(ROOK_MAGICS) as f64 / 1024.0);
    println!(
        "Bishop size: {:.2}kB",
        calc_size(BISHOP_MAGICS) as f64 / 1024.0
    );
    println!(
        "Total: {:.2}kB",
        (calc_size(ROOK_MAGICS) + calc_size(BISHOP_MAGICS)) as f64 / 1024.0
    )
}

fn calc_size(arr: [magics::Magic; 64]) -> usize {
    arr.iter()
        .map(|m| std::mem::size_of::<magics::Magic>() + std::mem::size_of_val(m.array))
        .sum::<usize>()
}
