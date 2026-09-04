use backend::consts::{BISHOP_RELEVANCY, ROOK_RELEVANCY};
use std::{collections::HashMap, sync::LazyLock};

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

const MAX_SIZE: usize = 16384;

// Tuple of (blocker, attacks) for each square
static BISHOP_BLOCKER_ATTACKS: LazyLock<[Vec<(u64, u64)>; 64]> = LazyLock::new(|| {
    std::array::from_fn(|square| {
        let relevancy = BISHOP_RELEVANCY[square];
        let mut blockers = relevancy;
        (0..1usize << relevancy.count_ones())
            .map(|_| {
                blockers = blockers.wrapping_sub(1) & relevancy;
                let attacks = calculate_attacks(square, blockers, &BISHOP_DIRS);
                (blockers, attacks)
            })
            .collect()
    })
});

static ROOK_BLOCKER_ATTACKS: LazyLock<[Vec<(u64, u64)>; 64]> = LazyLock::new(|| {
    std::array::from_fn(|square| {
        let relevancy = ROOK_RELEVANCY[square];
        let mut blockers = relevancy;
        (0..1usize << relevancy.count_ones())
            .map(|_| {
                blockers = blockers.wrapping_sub(1) & relevancy;
                let attacks = calculate_attacks(square, blockers, &ROOK_DIRS);
                (blockers, attacks)
            })
            .collect()
    })
});

fn main() {}

fn check_magic(magic: u64, shift: u32, is_rook: bool) -> Option<[Option<u64>; MAX_SIZE]> {
    let blocker_attacks = if is_rook {
        &*ROOK_BLOCKER_ATTACKS
    } else {
        &*BISHOP_BLOCKER_ATTACKS
    };
    let mut array: [Option<u64>; MAX_SIZE] = [None; MAX_SIZE];
    for square in 0..64 {
        for (blocker, attack) in &blocker_attacks[square] {
            let hash = (blocker.wrapping_mul(magic) >> shift) as usize;
            if let Some(other_attack) = array[hash] {
                if other_attack != *attack {
                    return None;
                }
            } else {
                array[hash] = Some(*attack);
            }
        }
    }
    Some(array)
}

fn calculate_attacks(square: usize, blockers: u64, directions: &[(i32, i32)]) -> u64 {
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

fn compress_magic(magic: u64, shift: u32, is_rook: bool, map: &HashMap<u64, u64>) -> u32 {
    // Given a hashmap and a shift, check if we can increase the shift to reduce highest number in map
    // check if 1 rshift on all keys in map results in a valid map
    // valid = either shift into empty key or into key where val are same
    loop {
        try_rshift_map(&map);
    }
    0
}

fn check_rshift_map(map: &HashMap<u64, u64>) -> Result<(), ()> {
    Ok(())
}
