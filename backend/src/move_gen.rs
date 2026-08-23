use crate::consts::KNIGHT_MOVES;

/// Given a bitboard of a single knight,
/// Return the bitboard of all possible knight moves
pub fn get_knight_moves(knight: u64) -> u64 {
    return KNIGHT_MOVES[knight.trailing_zeros() as usize];
}
