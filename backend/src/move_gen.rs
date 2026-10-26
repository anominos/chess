use crate::board::{Board, Colour, Piece};
use crate::consts::*;

pub fn gen_moves(board: &Board, turn: Colour) -> [u64; 64] {
    let mut pseudo = gen_pseudolegal_moves(&board, turn);
    // Make sure the move doesn't leave the king in check
    for (from, to, promote) in iter_moves(&pseudo.clone(), &board) {
        let post_move = board.make_move(from, to, promote);
        // attack set of opposition after move
        let atk_set = get_attack_set(&post_move, turn.other());
        let k_square = match turn {
            Colour::W => post_move.w,
            Colour::B => post_move.b,
        }[Piece::K as usize];
        debug_assert!(k_square.count_ones() == 1);
        if atk_set & k_square != 0 {
            // This move leaves us in check, remove
            let sq = from.trailing_zeros() as usize;
            pseudo[sq] &= !to;
        }
    }
    // Invalidate castle if we moved through check (check if the adjacent square is still valid)
    // Also invalidate if we are in check
    let atk_set = get_attack_set(&board, turn.other());
    match turn {
        Colour::W => {
            let in_check = (board.w[Piece::K as usize] & atk_set) != 0;
            if board.w[Piece::K as usize] == E1 {
                let k_idx = E1.trailing_zeros() as usize;
                if in_check || ((pseudo[k_idx] & C1 != 0) && (pseudo[k_idx] & D1 == 0)) {
                    pseudo[k_idx] &= !C1;
                }
                if in_check || ((pseudo[k_idx] & G1 != 0) && (pseudo[k_idx] & F1 == 0)) {
                    pseudo[k_idx] &= !G1;
                }
            }
        }
        Colour::B => {
            let in_check = (board.b[Piece::K as usize] & atk_set) != 0;
            if board.b[Piece::K as usize] == E8 {
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
    board: &Board,
) -> impl Iterator<Item = (u64, u64, Option<Piece>)> {
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
            if (fr & board.w[Piece::P as usize] != 0 && to & R8 != 0)
                || (fr & board.b[Piece::P as usize] != 0 && to & R1 != 0)
            {
                [
                    Some((fr, to, Some(Piece::Q))),
                    Some((fr, to, Some(Piece::N))),
                    Some((fr, to, Some(Piece::R))),
                    Some((fr, to, Some(Piece::B))),
                ]
            } else {
                [Some((fr, to, None)), None, None, None]
            }
            .into_iter()
            .flatten()
        })
    })
}

fn gen_pseudolegal_moves(board: &Board, turn: Colour) -> [u64; 64] {
    let mut moves = [0; 64];
    let exclude = match turn {
        Colour::W => board.white(),
        Colour::B => board.black(),
    };
    let occ = board.occupancy();
    for p in Piece::PIECES {
        for sq in board.squares_by_piece(&p, turn) {
            let possible_moves = match p {
                Piece::P => get_pawn_moves(sq, turn, &board),
                Piece::B => get_bishop_moves(sq, occ),
                Piece::N => get_knight_moves(sq),
                Piece::R => get_rook_moves(sq, occ),
                Piece::Q => get_queen_moves(sq, occ),
                Piece::K => get_king_moves(sq, turn, &board),
            };
            // filter out self captures
            moves[sq.trailing_zeros() as usize] = possible_moves & !exclude;
        }
    }
    moves
}

pub fn get_attack_set(board: &Board, turn: Colour) -> u64 {
    const LEFT_MASK: u64 = 0x007F_7F7F_7F7F_7F00u64;
    const RIGHT_MASK: u64 = 0x00FE_FEFE_FEFE_FE00u64;
    let mut atks = 0;
    match turn {
        Colour::W => {
            atks |= (board.w[Piece::P as usize] & LEFT_MASK) << 9;
            atks |= (board.w[Piece::P as usize] & RIGHT_MASK) << 7;
        }
        Colour::B => {
            atks |= (board.b[Piece::P as usize] & LEFT_MASK) >> 7;
            atks |= (board.b[Piece::P as usize] & RIGHT_MASK) >> 9;
        }
    }
    for sq in board.squares_by_piece(&Piece::N, turn) {
        atks |= get_knight_moves(sq);
    }
    let occ = board.occupancy();
    for sq in board.squares_by_piece(&Piece::B, turn) {
        atks |= get_bishop_moves(sq, occ);
    }
    for sq in board.squares_by_piece(&Piece::R, turn) {
        atks |= get_rook_moves(sq, occ);
    }
    for sq in board.squares_by_piece(&Piece::Q, turn) {
        atks |= get_queen_moves(sq, occ);
    }
    for sq in board.squares_by_piece(&Piece::K, turn) {
        atks |= KING_MOVES[sq.trailing_zeros() as usize];
    }
    atks
}

pub fn get_pawn_moves(piece: u64, colour: Colour, board: &Board) -> u64 {
    let captures = match colour {
        Colour::W => &WHITE_PAWN_CAPTURES,
        Colour::B => &BLACK_PAWN_CAPTURES,
    };
    let sq_idx = piece.trailing_zeros() as usize;
    let row = sq_idx / 8;
    let mut moves = 0;
    if (1..7).contains(&row) {
        moves = match colour {
            Colour::W => piece << 8,
            Colour::B => piece >> 8,
        } & !board.occupancy();
    }
    if moves != 0 && colour == Colour::W && row == 1 {
        // no piece in the way, we are white and we are on the 2nd row,
        // add jump
        moves |= piece << 16 & !board.occupancy();
    } else if moves != 0 && colour == Colour::B && row == 6 {
        // no piece in the way, we are black and we are on the 7th row,
        // add jump
        moves |= piece >> 16 & !board.occupancy();
    }
    moves | (captures[sq_idx] & (board.occupancy() | board.en_passant))
}

pub fn get_knight_moves(piece: u64) -> u64 {
    KNIGHT_MOVES[piece.trailing_zeros() as usize]
}

pub fn get_king_moves(piece: u64, turn: Colour, board: &Board) -> u64 {
    // Castling moves
    let slide_moves = get_rook_moves(piece, board.occupancy());
    let mut castle_moves = 0u64;
    match turn {
        Colour::W => {
            if (board.castle_rights & Board::WQ) != 0 && (slide_moves & A1) != 0 {
                castle_moves |= C1;
            }
            if (board.castle_rights & Board::WK) != 0 && (slide_moves & H1) != 0 {
                castle_moves |= G1;
            }
        }
        Colour::B => {
            if (board.castle_rights & Board::BQ) != 0 && (slide_moves & A8) != 0 {
                castle_moves |= C8;
            }
            if (board.castle_rights & Board::BK) != 0 && (slide_moves & H8) != 0 {
                castle_moves |= G8;
            }
        }
    }
    KING_MOVES[piece.trailing_zeros() as usize] | castle_moves
}

pub fn get_bishop_moves(piece: u64, occ: u64) -> u64 {
    let piece_idx = piece.trailing_zeros() as usize;
    let magic = &BISHOP_MAGICS[piece_idx];
    magic.array[(((occ | magic.mask).wrapping_mul(magic.magic)) >> magic.shift) as usize]
}

pub fn get_rook_moves(piece: u64, occ: u64) -> u64 {
    let piece_idx = piece.trailing_zeros() as usize;
    let magic = &ROOK_MAGICS[piece_idx];
    magic.array[(((occ | magic.mask).wrapping_mul(magic.magic)) >> magic.shift) as usize]
}

pub fn get_queen_moves(piece: u64, occ: u64) -> u64 {
    get_bishop_moves(piece, occ) | get_rook_moves(piece, occ)
}
