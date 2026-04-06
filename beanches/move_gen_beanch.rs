// benches/move_gen_bench.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use chess_q::engine::move_gen::MoveGenerator;
use chess_q::state::game_state::GameState;

fn bench_move_generation(c: &mut Criterion) {
    let move_gen = MoveGenerator::new();
    let state = GameState::initial();
    
    c.bench_function("move_gen_initial", |b| {
        b.iter(|| {
            let moves = move_gen.generate_moves(black_box(&state));
            black_box(moves.len())
        })
    });
}

criterion_group!(benches, bench_move_generation);
criterion_main!(benches);
