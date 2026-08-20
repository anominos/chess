use std::fmt;

#[repr(usize)]
enum Piece {
    P,
    N,
    B,
    R,
    Q,
    K,
    COUNT,
}

#[derive(Default)]
pub struct Board {
    w: [u64; Piece::COUNT as usize],
    b: [u64; Piece::COUNT as usize],
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in (0..8).rev() {
            for col in 0..8 {
                let mask: u64 = 1 << (row * 8 + col);
                #[rustfmt::skip]
                write!(f, "{}", {
                    if self.w[Piece::P as usize] & mask != 0 {"p"}
                    else if self.w[Piece::N as usize] & mask != 0 {"n"}
                    else if self.w[Piece::B as usize] & mask != 0 {"b"}
                    else if self.w[Piece::R as usize] & mask != 0 {"r"}
                    else if self.w[Piece::Q as usize] & mask != 0 {"q"}
                    else if self.w[Piece::K as usize] & mask != 0 {"k"}
                    else if self.b[Piece::P as usize] & mask != 0 {"P"}
                    else if self.b[Piece::N as usize] & mask != 0 {"N"}
                    else if self.b[Piece::B as usize] & mask != 0 {"B"}
                    else if self.b[Piece::R as usize] & mask != 0 {"R"}
                    else if self.b[Piece::Q as usize] & mask != 0 {"Q"}
                    else if self.b[Piece::K as usize] & mask != 0 {"K"}
                    else {"."}
                })?;
            }
        }
        Ok(())
    }
}

impl Board {
    #[cfg(debug_assertions)]
    pub fn validate(&self) {
        let mut join: u64 = std::u64::MAX;
        for i in 0..Piece::COUNT as usize {
            join &= self.w[i];
        }
        debug_assert!(join == 0)
    }
    #[cfg(not(debug_assertions))]
    pub fn validate(&self) {}
}
