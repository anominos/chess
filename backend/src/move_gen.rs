use crate::board;
use crate::consts::*;

pub fn gen_moves(board: &board::Board, turn: &board::Colour) -> [u64; 64] {
    let mut pseudo = gen_pseudolegal_moves(&board, &turn);
    // Make sure the move doesn't leave the king in check
    for (from, to, promote) in iter_moves(&pseudo.clone(), &board) {
        let post_move = board.make_move(from, to, promote);
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
    // Invalidate castle if we moved through check (check if the adjacent square is still valid)
    // Also invalidate if we are in check
    let atk_set = gen_pseudolegal_moves(&board, &turn.other())
        .iter()
        .fold(0, |acc, x| acc | x);
    match turn {
        board::Colour::W => {
            let in_check = (board.w[board::Piece::K as usize] & atk_set) != 0;
            if board.w[board::Piece::K as usize] == E1 {
                let k_idx = E1.trailing_zeros() as usize;
                if in_check || ((pseudo[k_idx] & C1 != 0) && (pseudo[k_idx] & D1 == 0)) {
                    pseudo[k_idx] &= !C1;
                }
                if in_check || ((pseudo[k_idx] & G1 != 0) && (pseudo[k_idx] & F1 == 0)) {
                    pseudo[k_idx] &= !G1;
                }
            }
        }
        board::Colour::B => {
            let in_check = (board.b[board::Piece::K as usize] & atk_set) != 0;
            if board.b[board::Piece::K as usize] == E8 {
                let k_idx = E8.trailing_zeros() as usize;
                if in_check || ((pseudo[k_idx] & C8 != 0) && (pseudo[k_idx] & D8 == 0)) {
                    pseudo[k_idx] &= !C8;
                }
                if in_check || ((pseudo[k_idx] & G8 != 0) && (pseudo[k_idx] & F8 == 0)) {
                    pseudo[k_idx] &= !G8;
                }
            }
        }
    };

    pseudo
}

pub fn iter_moves(
    moves: &[u64; 64],
    board: &board::Board,
) -> impl Iterator<Item = (u64, u64, Option<board::Piece>)> {
    const R1: u64 = A1 | B1 | C1 | D1 | E1 | F1 | G1 | H1;
    const R8: u64 = A8 | B8 | C8 | D8 | E8 | F8 | G8 | H8;
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
        .flat_map(|(fr, to)| {
            if (fr & board.w[board::Piece::P as usize] != 0 && to & R8 != 0)
                || (fr & board.b[board::Piece::P as usize] != 0 && to & R1 != 0)
            {
                [
                    Some((fr, to, Some(board::Piece::Q))),
                    Some((fr, to, Some(board::Piece::N))),
                    Some((fr, to, Some(board::Piece::R))),
                    Some((fr, to, Some(board::Piece::B))),
                ]
            } else {
                [Some((fr, to, None)), None, None, None]
            }
            .into_iter()
            .flatten()
        })
    })
}

fn gen_pseudolegal_moves(board: &board::Board, turn: &board::Colour) -> [u64; 64] {
    let mut moves = [0; 64];
    let exclude = match turn {
        board::Colour::W => board.white(),
        board::Colour::B => board.black(),
    };
    for p in board::Piece::PIECES {
        for sq in board.squares_by_piece(&p, &turn) {
            let possible_moves = match p {
                board::Piece::P => get_pawn_moves(sq, &turn, &board),
                board::Piece::B => get_bishop_moves(sq, &board),
                board::Piece::N => get_knight_moves(sq),
                board::Piece::R => get_rook_moves(sq, &board),
                board::Piece::Q => get_queen_moves(sq, &board),
                board::Piece::K => get_king_moves(sq, &turn, &board),
            };
            // filter out self captures
            moves[sq.trailing_zeros() as usize] = possible_moves & !exclude;
        }
    }
    moves
}

fn get_pawn_moves(piece: u64, colour: &board::Colour, board: &board::Board) -> u64 {
    let captures = match colour {
        board::Colour::W => &WHITE_PAWN_CAPTURES,
        board::Colour::B => &BLACK_PAWN_CAPTURES,
    };
    let sq_idx = piece.trailing_zeros() as usize;
    let row = sq_idx / 8;
    let mut moves = 0;
    if (1..7).contains(&row) {
        moves = match colour {
            board::Colour::W => piece << 8,
            board::Colour::B => piece >> 8,
        } & !board.occupancy();
    }
    if moves != 0 && *colour == board::Colour::W && row == 1 {
        // no piece in the way, we are white and we are on the 2nd row,
        // add jump
        moves |= piece << 16 & !board.occupancy();
    } else if moves != 0 && *colour == board::Colour::B && row == 6 {
        // no piece in the way, we are black and we are on the 7th row,
        // add jump
        moves |= piece >> 16 & !board.occupancy();
    }
    moves | (captures[sq_idx] & (board.occupancy() | board.en_passant))
}

fn get_knight_moves(piece: u64) -> u64 {
    KNIGHT_MOVES[piece.trailing_zeros() as usize]
}

fn get_king_moves(piece: u64, turn: &board::Colour, board: &board::Board) -> u64 {
    // Castling moves
    let slide_moves = get_rook_moves(piece, board);
    let mut castle_moves = 0u64;
    match turn {
        board::Colour::W => {
            if (board.castle_rights & board::Board::WQ) != 0 && (slide_moves & A1) != 0 {
                castle_moves |= C1;
            }
            if (board.castle_rights & board::Board::WK) != 0 && (slide_moves & H1) != 0 {
                castle_moves |= G1;
            }
        }
        board::Colour::B => {
            if (board.castle_rights & board::Board::BQ) != 0 && (slide_moves & A8) != 0 {
                castle_moves |= C8;
            }
            if (board.castle_rights & board::Board::BK) != 0 && (slide_moves & H8) != 0 {
                castle_moves |= G8;
            }
        }
    }
    KING_MOVES[piece.trailing_zeros() as usize] | castle_moves
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
