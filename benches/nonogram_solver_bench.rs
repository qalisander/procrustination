use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_sandbox::nonogram_solvers::tests::CLUES_15;
use rust_sandbox::nonogram_solvers::{nonogram_solver_bitsets, nonogram_solver_slice};

//https://bheisler.github.io/criterion.rs/book/user_guide/advanced_configuration.html

fn nonogram_solover_bench(c: &mut Criterion) {
    c.benchmark_group("Nonogram solver benches")
        .sample_size(20)
        // time:   [244.41 ms 245.48 ms 246.44 ms]
        .bench_function("nonogram_solover_bitsets", |b| {
            b.iter(|| nonogram_solver_bitsets::solve_nonogram(CLUES_15))
        })
        // time:   [64.710 ms 65.040 ms 65.472 ms]
        .bench_function("nonogram_solover_slice", |b| {
            b.iter(|| nonogram_solver_slice::solve_nonogram(CLUES_15))
        });
}

criterion_group!(benches, nonogram_solover_bench);
criterion_main!(benches);
