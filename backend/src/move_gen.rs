use crate::board;
use crate::consts::*;

pub fn gen_moves(board: &board::Board, turn: &board::Colour) -> [u64; 64] {
    let mut pseudo = gen_pseudolegal_moves(board, turn);
    // Make sure the move doesn't leave the king in check
    for (from, to) in iter_moves(&pseudo.clone()) {
        let post_move = board.make_move(from, to);
        // attack set of opposition after move
        let atk_set = gen_pseudolegal_moves(&post_move, &turn.other())
            .iter()
            .fold(0, |acc, x| acc | x);
        let k_square = match turn {
            board::Colour::W => post_move.w,
            board::Colour::B => post_move.b,
        }[board::Piece::K as usize];
        debug_assert!(k_square.count_ones() == 1);
        if atk_set & k_square != 0 {
            // This move leaves us in check, remove
            let sq = from.trailing_zeros() as usize;
            pseudo[sq] &= !to;
        }
    }
    pseudo
}

pub fn iter_moves(moves: &[u64; 64]) -> impl Iterator<Item = (u64, u64)> {
    (0..64).flat_map(|sq| {
        let from = 1u64 << sq;
        let mut m = moves[sq];
        std::iter::from_fn(move || {
            if m == 0 {
                None
            } else {
                let to = m & m.wrapping_neg();
                m &= m - 1;
                Some((from, to))
            }
        })
    })
}

fn gen_pseudolegal_moves(board: &board::Board, turn: &board::Colour) -> [u64; 64] {
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
            let exclude = match turn {
                board::Colour::W => board.white(),
                board::Colour::B => board.black(),
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
