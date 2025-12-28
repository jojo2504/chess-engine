use chess_engine::engine::models::{board::{Chessboard, Color}, piece::Piece};
use criterion::{criterion_group, criterion_main, Criterion, black_box};
use std::time::Duration;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

// Single array approach
struct ArrayBoard {
    pieces: [u64; 12],
}

impl ArrayBoard {
    fn new() -> Self {
        Self { pieces: [0; 12] }
    }
    
    #[inline]
    fn get_piece(&self, color: Color, piece: Piece) -> u64 {
        self.pieces[color as usize * 6 + piece as usize]
    }
}

fn bench_array(board: &ArrayBoard, color: Color, piece: Piece) -> u64 {
    board.get_piece(color, piece)
}

fn generate_random_cases(count: usize) -> Vec<(Color, Piece)> {
    let mut rng = StdRng::seed_from_u64(42);
    let colors = [Color::White, Color::Black];
    let pieces = [
        Piece::Pawn, Piece::Knight, Piece::Bishop,
        Piece::Rook, Piece::Queen, Piece::King,
    ];
    
    (0..count)
        .map(|_| {
            let color = colors[rng.gen_range(0..2)];
            let piece = pieces[rng.gen_range(0..6)];
            (color, piece)
        })
        .collect()
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("chess_access_patterns");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(1000);
    
    let test_cases = generate_random_cases(10000);
    let chessboard = Chessboard::new();
    let array_board = ArrayBoard::new();
    
    group.bench_function("single_array_computed_index", |b| {
        let mut idx = 0;
        b.iter(|| {
            let (color, piece) = test_cases[idx % test_cases.len()];
            idx += 1;
            black_box(bench_array(&array_board, color, piece))
        })
    });
    
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);