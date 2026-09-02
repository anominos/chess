use backend::consts::{BISHOP_RELEVANCY, ROOK_RELEVANCY};
use std::collections::{HashMap, hash_map::Entry};

const ROOK_DIRS: [(i32, i32); 4] = [
    (1, 0),  // north
    (-1, 0), // south
    (0, 1),  // east
    (0, -1), // west
];

const BISHOP_DIRS: [(i32, i32); 4] = [
    (1, 1),   // NE
    (1, -1),  // NW
    (-1, 1),  // SE
    (-1, -1), // SW
];
fn main() {}

fn check_magic(magic: u64, shift: u32, is_rook: bool) -> Option<HashMap<u64, u64>> {
    let mut hash_to_atk: HashMap<u64, u64> = HashMap::with_capacity(16384);
    let relevancies = if is_rook {
        ROOK_RELEVANCY
    } else {
        BISHOP_RELEVANCY
    };
    let calc_dir = if is_rook { ROOK_DIRS } else { BISHOP_DIRS };
    for square in 0..64 {
        let relevancy = relevancies[square];
        let mut blockers = relevancy;
        loop {
            let hash = blockers.wrapping_mul(magic) >> shift;
            let attackers = calculate_attackers(square, blockers, &calc_dir);

            match hash_to_atk.entry(hash) {
                Entry::Vacant(e) => {
                    e.insert(attackers);
                }
                Entry::Occupied(e) if *e.get() != attackers => {
                    return None;
                }
                Entry::Occupied(_) => {}
            };

            if blockers == 0 {
                break;
            }
            blockers = (blockers - 1) & relevancy;
        }
    }
    Some(hash_to_atk)
}

fn calculate_attackers(square: usize, blockers: u64, directions: &[(i32, i32)]) -> u64 {
    let mut attacks = 0;

    let rank = (square / 8) as i32;
    let file = (square % 8) as i32;

    for &(dr, df) in directions {
        let mut r = rank + dr;
        let mut f = file + df;

        while (0..8).contains(&r) && (0..8).contains(&f) {
            let target = (r * 8 + f) as usize;

            attacks |= 1u64 << target;

            if blockers & (1u64 << target) != 0 {
                break;
            }

            r += dr;
            f += df;
        }
    }

    attacks
}
/*
m = gen random number
d = {}
For each piece (0..64)
    get relevancy
        n = num bits in relevancy
        get combination for relevancy (2^n) -> 2^14
            for each comb:
            - calculate attack set
            - d[attack set] -> [hash = comb * m >> 11/13]





*/
