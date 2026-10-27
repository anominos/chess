use crate::{
    board::{Board, Colour, Piece},
    engine::heatmap::PIECE_HEATMAPS_EARLY,
    move_gen::{gen_moves, iter_moves},
};

const PIECE_VALUES: [i32; 6] = [100, 300, 330, 500, 900, 20000];
const MATE: i32 = 100_000;

pub fn evaluate(board: &Board, turn: Colour, depth: u8) -> i32 {
    if depth == 0 {
        return evaluate_board(board);
    }
    let moves = gen_moves(board, turn);
    let eval_iter = iter_moves(&moves, board).map(|(from, to, promote)| {
        evaluate(&board.make_move(from, to, promote), turn.other(), depth - 1)
    });
    match turn {
        Colour::W => eval_iter
            .max()
            .unwrap_or(if board.in_check(turn) { -MATE } else { 0 }),
        Colour::B => eval_iter
            .min()
            .unwrap_or(if board.in_check(turn) { MATE } else { 0 }),
    }
}

pub fn evaluate_board(board: &Board) -> i32 {
    let mut eval: i32 = 0;
    for piece in Piece::PIECES {
        let idx = piece as usize;
        eval += eval_piece(
            board.w[idx],
            PIECE_HEATMAPS_EARLY[idx],
            PIECE_VALUES[idx],
            true,
        );
        eval -= eval_piece(
            board.b[idx],
            PIECE_HEATMAPS_EARLY[idx],
            PIECE_VALUES[idx],
            false,
        );
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
