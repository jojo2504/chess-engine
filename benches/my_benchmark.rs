use criterion::{criterion_group, criterion_main, Criterion, black_box};
use lib::engine::models::board::Chessboard;
use std::time::Duration;
use lib::perft;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("perft");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(50);
    
    group.bench_function("perft_depth_5", |b| {
        b.iter(|| {
            let mut chessboard = Chessboard::new();
            std::hint::black_box(perft(&mut chessboard, 5))
        })
    });
    
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);