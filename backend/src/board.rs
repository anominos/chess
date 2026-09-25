use crate::consts::*;
use std::fmt;

#[repr(usize)]
#[derive(Clone, Copy)]
pub enum Piece {
    P,
    N,
    B,
    R,
    Q,
    K,
}
impl Piece {
    pub const COUNT: usize = 6;
    pub const PIECES: [Piece; 6] = [Piece::P, Piece::N, Piece::B, Piece::R, Piece::Q, Piece::K];
}

#[repr(usize)]
#[derive(PartialEq, Clone, Copy)]
pub enum Colour {
    B,
    W,
}

#[derive(Clone)]
pub struct Board {
    pub w: [u64; Piece::COUNT],
    pub b: [u64; Piece::COUNT],
}

impl Board {
    pub fn default() -> Self {
        Self {
            w: [255u64 << 8, B1 | G1, C1 | F1, A1 | H1, D1, E1],
            b: [255u64 << (6 * 8), B8 | G8, C8 | F8, A8 | H8, D8, E8],
        }
    }

    pub fn white(&self) -> u64 {
        (0..Piece::COUNT).fold(0, |acc, x| acc | self.w[x])
    }

    pub fn black(&self) -> u64 {
        (0..Piece::COUNT).fold(0, |acc, x| acc | self.b[x])
    }

    pub fn occupancy(&self) -> u64 {
        self.white() | self.black()
    }

    pub fn make_move(&self, from: u64, to: u64) -> Self {
        let mut new_board = self.clone();
        let (fr_side, to_side) = if from & self.white() != 0 {
            (&mut new_board.w, &mut new_board.b)
        } else if from & self.black() != 0 {
            (&mut new_board.b, &mut new_board.w)
        } else {
            panic!("From not occupied");
        };
        if let Some(b) = fr_side.iter_mut().find(|b| **b & from != 0) {
            *b = (*b & !from) | to;
        }
        if let Some(b) = to_side.iter_mut().find(|b| **b & to != 0) {
            // capture
            *b = *b & !to;
        }
        self.validate();
        new_board
    }

    pub fn squares_by_piece(&self, piece: &Piece, colour: &Colour) -> impl Iterator<Item = u64> {
        let mut bitboard = match colour {
            Colour::W => &self.w,
            Colour::B => &self.b,
        }[*piece as usize];
        std::iter::from_fn(move || {
            if bitboard == 0 {
                None
            } else {
                let bit = bitboard & bitboard.wrapping_neg();
                bitboard &= bitboard - 1;
                Some(bit)
            }
        })
    }

    #[cfg(debug_assertions)]
    pub fn validate(&self) {
        let mut seen: u64 = 0;
        for i in 0..Piece::COUNT {
            debug_assert!(seen & self.w[i] == 0);
            seen |= self.w[i];
            debug_assert!(seen & self.b[i] == 0);
            seen |= self.b[i];
        }
    }
    #[cfg(not(debug_assertions))]
    pub fn validate(&self) {}
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
            writeln!(f, "")?;
        }
        Ok(())
    }
}
