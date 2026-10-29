use crate::{
    board::{Board, Colour, Piece},
    engine::heatmap::{PIECE_HEATMAPS_EARLY, PIECE_HEATMAPS_LATE},
    move_gen::{gen_moves, iter_moves},
};

const PIECE_VALUES: [i32; 6] = [100, 320, 330, 500, 900, 20000];
const MATE: i32 = 100_000;

pub fn evaluate(board: &Board, turn: Colour, depth: u8) -> i32 {
    _evaluate(board, turn, depth, -MATE - 1, MATE + 1)
}

fn _evaluate(board: &Board, turn: Colour, depth: u8, mut alpha: i32, beta: i32) -> i32 {
    if depth == 0 {
        return match turn {
            Colour::W => evaluate_board(board),
            Colour::B => -evaluate_board(board),
        };
    }
    let mut has_moves = false;
    for (from, to, promote) in iter_moves(&gen_moves(board, turn), &board) {
        has_moves = true;
        let child_eval = -_evaluate(
            &board.make_move(from, to, promote),
            turn.other(),
            depth - 1,
            -beta,
            -alpha,
        );
        if child_eval >= beta {
            return beta;
        }
        if child_eval > alpha {
            alpha = child_eval;
        }
    }
    if !has_moves {
        if board.in_check(turn) {
            return -MATE + depth as i32;
        } else {
            0;
        }
    }
    alpha
}

fn evaluate_board(board: &Board) -> i32 {
    let mut eval: i32 = 0;
    let heatmap = if board.phase >= 24 {
        PIECE_HEATMAPS_EARLY
    } else {
        PIECE_HEATMAPS_LATE
    };
    for piece in Piece::PIECES {
        let idx = piece as usize;
        eval += eval_piece(board.w[idx], heatmap[idx], PIECE_VALUES[idx], true);
        eval -= eval_piece(board.b[idx], heatmap[idx], PIECE_VALUES[idx], false);
    }
    eval
}

fn eval_piece(piece: u64, heatmap: &[i32], piece_val: i32, rev: bool) -> i32 {
    let mut eval = 0;
    let mut piece_cpy = piece;
    while piece_cpy != 0 {
        let sq_idx = piece_cpy.trailing_zeros() as usize;
        if rev {
            eval += heatmap[sq_idx ^ 56] + piece_val;
        } else {
            eval += heatmap[sq_idx] + piece_val;
        }
        piece_cpy &= piece_cpy - 1;
    }
    eval
}
