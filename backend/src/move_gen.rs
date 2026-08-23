use crate::consts::*;

/// Given a bitboard of a single knight,
/// Return the bitboard of all possible knight moves
pub fn get_knight_moves(piece: u64) -> u64 {
    return KNIGHT_MOVES[piece.trailing_zeros() as usize];
}

pub fn get_king_moves(piece: u64) -> u64 {
    return KING_MOVES[piece.trailing_zeros() as usize];
}

pub fn get_bishop_moves(piece: u64) -> u64 {
    return BISHOP_MOVES[piece.trailing_zeros() as usize];
}

pub fn get_rook_moves(piece: u64) -> u64 {
    return ROOK_MOVES[piece.trailing_zeros() as usize];
}

pub fn get_queen_moves(piece: u64) -> u64 {
    return ROOK_MOVES[piece.trailing_zeros() as usize]
        | BISHOP_MOVES[piece.trailing_zeros() as usize];
}
