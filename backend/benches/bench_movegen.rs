use backend::{consts::*, *};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
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
            move_gen::iter_moves(&move_gen::gen_moves(&board, &colour), &board)
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
            |pos| black_box(move_gen::gen_moves(&pos.board, &pos.colour)),
            criterion::BatchSize::SmallInput,
        )
    });
}

pub fn criterion_pawns(c: &mut Criterion) {
    let fens = [
        ("8/8/8/8/8/8/3P4/8 w KQkq - - -", D2, "double push"),
        ("8/8/8/8/3P4/8/8/8 w KQkq - - -", D4, "push only"),
        ("8/8/8/8/8/4p3/3P4/8 w KQkq - - -", D2, "capture"),
        ("8/8/8/8/8/3p4/3P4/8 w KQkq - - -", D2, "blocked push"),
        ("8/8/8/3pP3/8/8/8/8 w KQkq d6 - -", E5, "en passant"),
    ];
    let mut group = c.benchmark_group("pawns");
    for (fen, piece, name) in fens {
        let (b, c) = board::Board::from_fen(fen).unwrap();
        group.bench_with_input(
            BenchmarkId::from_parameter(name),
            &(b, c, piece),
            |b, (board, col, piece)| {
                b.iter(|| black_box(move_gen::get_pawn_moves(*piece, &col, &board)));
            },
        );
    }
}

pub fn criterion_knights(c: &mut Criterion) {
    let (fen, sq) = ("8/8/8/4N3/8/8/8/8 w KQkq - - -", E5);
    let mut group = c.benchmark_group("pawns");
    let (b, c) = board::Board::from_fen(fen).unwrap();
    group.bench_function("knight", |f| {
        f.iter(|| black_box(move_gen::get_pawn_moves(sq, &c, &b)));
    });
}

criterion_group!(
    benches,
    criterion_movegen,
    criterion_pawns,
    criterion_knights
);
criterion_main!(benches);
