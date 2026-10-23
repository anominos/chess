use backend::{consts::*, *};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::hint::black_box;

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
                b.iter(|| black_box(move_gen::get_pawn_moves(*piece, *col, &board)));
            },
        );
    }
}

pub fn criterion_knights(c: &mut Criterion) {
    let (fen, sq) = ("8/8/8/4N3/8/8/8/8 w KQkq - - -", E5);
    let mut group = c.benchmark_group("pawns");
    let (b, c) = board::Board::from_fen(fen).unwrap();
    group.bench_function("knight", |f| {
        f.iter(|| black_box(move_gen::get_pawn_moves(sq, c, &b)));
    });
}

criterion_group!(benches, criterion_pawns, criterion_knights);
criterion_main!(benches);
