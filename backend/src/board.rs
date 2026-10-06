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
impl Colour {
    pub fn other(&self) -> Self {
        match self {
            Colour::B => Colour::W,
            Colour::W => Colour::B,
        }
    }
}

#[derive(Clone)]
pub struct Board {
    pub w: [u64; Piece::COUNT],
    pub b: [u64; Piece::COUNT],
    pub castle_rights: u8,
    pub en_passant: u64, // if last move was pawn jump, mark intermediate here
}

impl Board {
    pub const WQ: u8 = 1u8 << 0;
    pub const WK: u8 = 1u8 << 1;
    pub const BQ: u8 = 1u8 << 2;
    pub const BK: u8 = 1u8 << 3;

    pub fn default() -> Self {
        Self {
            w: [255u64 << 8, B1 | G1, C1 | F1, A1 | H1, D1, E1],
            b: [255u64 << (6 * 8), B8 | G8, C8 | F8, A8 | H8, D8, E8],
            castle_rights: Self::WQ | Self::WK | Self::BQ | Self::BK,
            en_passant: 0u64,
        }
    }

    pub fn from_fen(fen: &str) -> Result<(Self, Colour), &'static str> {
        let &[pieces, side, castling, en_passant, _halfmove, _fullmove] =
            fen.split_ascii_whitespace().collect::<Vec<_>>().as_slice()
        else {
            return Err("Error unpacking args");
        };
        let mut board = Self {
            w: [0; 6],
            b: [0; 6],
            castle_rights: 0,
            en_passant: 0,
        };

        // Pieces
        for (row, chars) in pieces.split("/").enumerate() {
            let mut col = 0;
            for chr in chars.chars() {
                if let Some(empty) = chr.to_digit(10) {
                    col += empty;
                } else {
                    let arr = if chr.is_ascii_lowercase() {
                        &mut board.b
                    } else {
                        &mut board.w
                    };
                    let sq = 1u64 << ((7 - row as u32) * 8 + col);
                    match chr.to_ascii_lowercase() {
                        'p' => arr[Piece::P as usize] |= sq,
                        'n' => arr[Piece::N as usize] |= sq,
                        'b' => arr[Piece::B as usize] |= sq,
                        'r' => arr[Piece::R as usize] |= sq,
                        'q' => arr[Piece::Q as usize] |= sq,
                        'k' => arr[Piece::K as usize] |= sq,
                        _ => return Err("invalid pieces"),
                    };
                    col += 1;
                }
            }
        }

        // Side
        let turn = match side {
            "w" => Colour::W,
            "b" => Colour::B,
            _ => return Err("invalid turn"),
        };

        // Castling right
        if castling.contains('K') {
            board.castle_rights |= Self::WK;
        }
        if castling.contains('Q') {
            board.castle_rights |= Self::WQ;
        }
        if castling.contains('k') {
            board.castle_rights |= Self::BK;
        }
        if castling.contains('q') {
            board.castle_rights |= Self::BQ;
        }

        // en passant
        if en_passant != "-" {
            if en_passant.len() != 2 {
                return Err("invalid sq - too long");
            }
            let bytes = en_passant.as_bytes();
            let col = match bytes[0] {
                b'a'..=b'h' => bytes[0] - b'a',
                _ => return Err("invalid sq, col"),
            };
            let row = match bytes[1] {
                b'1'..=b'8' => bytes[1] - b'1',
                _ => return Err("invalid sq, row"),
            };
            board.en_passant = 1u64 << (row * 8 + col);
        }

        Ok((board, turn))
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
        new_board.castle_rights &= !match from {
            E1 => Board::WQ | Board::WK,
            A1 => Board::WQ,
            H1 => Board::WK,
            E8 => Board::BQ | Board::BK,
            A8 => Board::BQ,
            H8 => Board::BK,
            _ => 0,
        };
        let (fr_side, fr_opp) = if from & self.white() != 0 {
            (&mut new_board.w, &mut new_board.b)
        } else if from & self.black() != 0 {
            (&mut new_board.b, &mut new_board.w)
        } else {
            panic!("From not occupied");
        };
        // Check for castling
        if from == fr_side[Piece::K as usize] {
            if from == E1 {
                if to == C1 {
                    fr_side[Piece::R as usize] = fr_side[Piece::R as usize] & !A1 | D1;
                } else if to == G1 {
                    fr_side[Piece::R as usize] = fr_side[Piece::R as usize] & !H1 | F1;
                }
            } else if from == E8 {
                if to == C8 {
                    fr_side[Piece::R as usize] = fr_side[Piece::R as usize] & !A8 | D8;
                } else if to == G8 {
                    fr_side[Piece::R as usize] = fr_side[Piece::R as usize] & !H8 | F8;
                }
            }
        }
        // Check for en passant marking
        new_board.en_passant = 0;
        if from & fr_side[Piece::P as usize] != 0 {
            if to == from << 16 {
                new_board.en_passant = from << 8;
            } else if to == from >> 16 {
                new_board.en_passant = from >> 8;
            }
        }

        if let Some(b) = fr_side.iter_mut().find(|b| **b & from != 0) {
            *b = (*b & !from) | to;
        }
        // check for en_passant capture
        if (from & fr_side[Piece::P as usize]) != 0 && to == self.en_passant {
            // can remove both since the other square must be empty (opp just moved from there)
            fr_opp[Piece::P as usize] &= !(self.en_passant << 8 | self.en_passant >> 8);
        }
        if let Some(b) = fr_opp.iter_mut().find(|b| **b & to != 0) {
            // capture
            *b = *b & !to;
        }
        new_board.validate();
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
        debug_assert!(seen & self.en_passant == 0);
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
                    if self.w[Piece::P as usize] & mask != 0 {"P"}
                    else if self.w[Piece::N as usize] & mask != 0 {"N"}
                    else if self.w[Piece::B as usize] & mask != 0 {"B"}
                    else if self.w[Piece::R as usize] & mask != 0 {"R"}
                    else if self.w[Piece::Q as usize] & mask != 0 {"Q"}
                    else if self.w[Piece::K as usize] & mask != 0 {"K"}
                    else if self.b[Piece::P as usize] & mask != 0 {"p"}
                    else if self.b[Piece::N as usize] & mask != 0 {"n"}
                    else if self.b[Piece::B as usize] & mask != 0 {"b"}
                    else if self.b[Piece::R as usize] & mask != 0 {"r"}
                    else if self.b[Piece::Q as usize] & mask != 0 {"q"}
                    else if self.b[Piece::K as usize] & mask != 0 {"k"}
                    else {"."}
                })?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}
