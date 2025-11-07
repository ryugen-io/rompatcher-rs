use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rom_patcher_cli::OnlyMode;

/// Benchmark creating a Vec<OnlyMode> with single mode
fn bench_single_mode(c: &mut Criterion) {
    c.bench_function("single_mode_verify", |b| {
        b.iter(|| {
            let modes: Vec<OnlyMode> = vec![black_box(OnlyMode::Verify)];
            black_box(modes)
        });
    });
}

/// Benchmark creating a Vec<OnlyMode> with multiple modes
fn bench_multiple_modes(c: &mut Criterion) {
    c.bench_function("multiple_modes_verify_ra", |b| {
        b.iter(|| {
            let modes: Vec<OnlyMode> = vec![
                black_box(OnlyMode::Verify),
                black_box(OnlyMode::Ra),
            ];
            black_box(modes)
        });
    });
}

/// Benchmark checking if Vec contains a mode with any()
fn bench_mode_check_any(c: &mut Criterion) {
    let modes = vec![OnlyMode::Verify, OnlyMode::Ra];
    
    c.bench_function("mode_check_any_verify", |b| {
        b.iter(|| {
            black_box(modes.iter().any(|m| matches!(m, OnlyMode::Verify)))
        });
    });
}

/// Benchmark iterating over modes
fn bench_mode_iteration(c: &mut Criterion) {
    let modes = vec![OnlyMode::Verify, OnlyMode::Ra];
    
    c.bench_function("mode_iteration", |b| {
        b.iter(|| {
            for mode in black_box(&modes) {
                black_box(mode);
            }
        });
    });
}

/// Benchmark empty Vec check
fn bench_empty_check(c: &mut Criterion) {
    let empty_modes: Vec<OnlyMode> = vec![];
    let non_empty_modes = vec![OnlyMode::Verify];
    
    c.bench_function("empty_check_true", |b| {
        b.iter(|| black_box(empty_modes.is_empty()));
    });
    
    c.bench_function("empty_check_false", |b| {
        b.iter(|| black_box(non_empty_modes.is_empty()));
    });
}

criterion_group!(
    benches,
    bench_single_mode,
    bench_multiple_modes,
    bench_mode_check_any,
    bench_mode_iteration,
    bench_empty_check
);
criterion_main!(benches);
