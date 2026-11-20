use crate::board::{Board, Colour, Piece};
use std::fmt;

pub struct Game {
    pub board: Board,
    pub cur_turn: Colour,
    history: Vec<u64>, // vec of zobrist hashes of previous positiions
}
impl Game {
    pub fn default() -> Self {
        Self {
            board: Board::default(),
            cur_turn: Colour::W,
            history: Vec::new(),
        }
    }
    pub fn from_fen(fen: &str) -> Result<Self, &'static str> {
        let (board, cur_turn) = Board::from_fen(fen)?;
        Ok(Self {
            board,
            cur_turn,
            history: Vec::new(),
        })
    }
    pub fn make_move(&mut self, from: u64, to: u64, promote: Option<Piece>) {
        self.board = self.board.make_move(from, to, promote);
        self.cur_turn = self.cur_turn.other();
    }
}
impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.board)
    }
}
