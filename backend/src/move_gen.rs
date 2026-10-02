use crate::board;
use crate::consts::*;

struct Move {
    from: u64,
    tos: u64,
}

pub fn gen_moves(board: &board::Board, turn: &board::Colour) -> Vec<Move> {
    let mut pseudo = gen_pseudolegal_moves(board, turn);
    // Make sure the move doesn't leave the king in check
    for Move { from, tos: mut tos } in pseudo {
        for to in iter_tos(tos) {
            let post_move = board.make_move(from, to);
            // attack set of opposition after move
            let atk_set = gen_pseudolegal_moves(&post_move, &turn.other())
                .iter()
                .fold(0, |acc, x| acc | x.tos);
            let k_square = match turn {
                board::Colour::W => post_move.w,
                board::Colour::B => post_move.b,
            }[board::Piece::K as usize];
            debug_assert!(k_square.count_ones() == 1);
            if atk_set & k_square != 0 {
                // This move leaves us in check, remove
                let sq = from.trailing_zeros() as usize;
                tos &= !to;
            }
        }
    }
    let k_board =
    let mut k_to = pseudo.iter_mut().find(|m| )
    // Invalidate castle if we moved through check (check if the adjacent square is still valid)
    match turn {
        board::Colour::W => {
            if board.w[board::Piece::K as usize] == E1 {
                if (k_to & C1 != 0) && (k_to & D1 == 0) {
                    k_to &= !C1;
                }
                if (k_to & G1 != 0) && (k_to & F1 == 0) {
                    k_to &= !G1;
                }
            }
        }
        board::Colour::B => {
            if board.w[board::Piece::K as usize] == E8 {
                if (k_to & C8 != 0) && (k_to & D8 == 0) {
                    k_to &= !C8;
                }
                if (k_to & G8 != 0) && (k_to & F8 == 0) {
                    k_to &= !G8;
                }
            }
        }
    };

    pseudo
}

fn iter_tos(tos: u64) -> impl Iterator<Item = u64> {
    let mut m = tos;
    std::iter::from_fn(move || {
        if m == 0 {
            None
        } else {
            let to = m & m.wrapping_neg();
            m &= m - 1;
            Some(to)
        }
    })
}

fn gen_pseudolegal_moves(board: &board::Board, turn: &board::Colour) -> Vec<Move> {
    let mut moves = Vec::new();
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
            moves.push(Move {
                from: sq,
                tos: possible_moves & !exclude,
            });
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
