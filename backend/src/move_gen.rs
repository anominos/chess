use crate::board;
use crate::consts::*;

pub fn gen_pseudolegal_moves(board: &board::Board, turn: &board::Colour) -> [u64; 64] {
    let mut moves = [0; 64];
    for p in board::Piece::PIECES {
        for sq in board.squares_by_piece(&p, &turn) {
            let possible_moves = match p {
                board::Piece::P => get_pawn_moves(sq, &turn, &board),
                board::Piece::B => get_bishop_moves(sq, &board),
                board::Piece::N => get_knight_moves(sq),
                board::Piece::R => get_rook_moves(sq, &board),
                board::Piece::Q => get_queen_moves(sq, &board),
                board::Piece::K => get_king_moves(sq),
            };
            // filter out self captures
            let exclude = if *turn == board::Colour::W {
                board.white()
            } else {
                board.black()
            };
            moves[sq.trailing_zeros() as usize] = possible_moves & !exclude;
        }
    }
    moves
}

fn get_pawn_moves(piece: u64, colour: &board::Colour, board: &board::Board) -> u64 {
    let (&moves, &captures) = match colour {
        board::Colour::W => (&WHITE_PAWN_MOVES, &WHITE_PAWN_CAPTURES),
        board::Colour::B => (&BLACK_PAWN_MOVES, &BLACK_PAWN_CAPTURES),
    };
    let sq_idx = piece.trailing_zeros() as usize;
    moves[sq_idx] | (captures[sq_idx] & board.occupancy())
}

fn get_knight_moves(piece: u64) -> u64 {
    KNIGHT_MOVES[piece.trailing_zeros() as usize]
}

fn get_king_moves(piece: u64) -> u64 {
    KING_MOVES[piece.trailing_zeros() as usize]
}

fn get_bishop_moves(piece: u64, board: &board::Board) -> u64 {
    let piece_idx = piece.trailing_zeros() as usize;
    let magic = &BISHOP_MAGICS[piece_idx];
    magic.array
        [(((board.occupancy() | magic.mask).wrapping_mul(magic.magic)) >> magic.shift) as usize]
}

fn get_rook_moves(piece: u64, board: &board::Board) -> u64 {
    let piece_idx = piece.trailing_zeros() as usize;
    let magic = &ROOK_MAGICS[piece_idx];
    magic.array
        [(((board.occupancy() | magic.mask).wrapping_mul(magic.magic)) >> magic.shift) as usize]
}

fn get_queen_moves(piece: u64, board: &board::Board) -> u64 {
    get_bishop_moves(piece, board) | get_rook_moves(piece, board)
}
