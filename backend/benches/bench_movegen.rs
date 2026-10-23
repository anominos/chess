use backend::*;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

const FEN: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - - -";

struct Pos {
    board: board::Board,
    colour: board::Colour,
}

#[derive(Clone)]
struct DFS {
    stack: Vec<(board::Board, board::Colour, u8)>,
}

impl Iterator for DFS {
    type Item = Pos;
    fn next(&mut self) -> Option<Self::Item> {
        let (mut board, mut colour, mut depth);
        loop {
            (board, colour, depth) = self.stack.pop()?;
            if depth > 0 {
                break;
            }
        }
        for (from, to, promote) in
            move_gen::iter_moves(&move_gen::gen_moves(&board, colour), &board)
        {
            self.stack.push((
                board.make_move(from, to, promote),
                colour.other(),
                depth - 1,
            ));
        }
        Some(Pos { board, colour })
    }
}

pub fn criterion_movegen(c: &mut Criterion) {
    let (bd, cl) = board::Board::from_fen(FEN).unwrap();
    let mut boards = (DFS {
        stack: vec![(bd, cl, 5)],
    })
    .cycle();
    c.bench_function("movegen", move |b| {
        b.iter_batched(
            || boards.next().unwrap(),
            |pos| black_box(move_gen::gen_moves(&pos.board, pos.colour)),
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, criterion_movegen,);
criterion_main!(benches);
