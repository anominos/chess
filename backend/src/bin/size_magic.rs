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
    );
    println!("----");
    print_popcounts();
}

fn calc_size(arr: [magics::Magic; 64]) -> usize {
    arr.iter()
        .map(|m| std::mem::size_of::<magics::Magic>() + std::mem::size_of_val(m.array))
        .sum::<usize>()
}

fn print_popcounts() -> () {
    println!("B:");
    for i in 0..64 {
        let pc = BISHOP_MAGICS[i].magic.count_ones();
        print!("{:0>2}({: >4}) ", pc, BISHOP_MAGICS[i].array.len());
        if i % 8 == 7 {
            println!();
        }
    }
    println!("R:");
    for i in 0..64 {
        let pc = ROOK_MAGICS[i].magic.count_ones();
        print!("{:0>2}({: >4}) ", pc, ROOK_MAGICS[i].array.len());
        if i % 8 == 7 {
            println!();
        }
    }
}
