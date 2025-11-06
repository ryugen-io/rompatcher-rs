use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use rom_patcher_core::PatchFormat;
use rom_patcher_formats::ebp::EbpPatcher;

/// Generate a test EBP patch with JSON metadata
/// EBP is IPS-based with optional JSON metadata after EOF
fn generate_test_patch(rom_size: usize, patch_count: usize) -> Vec<u8> {
    let mut patch = Vec::new();
    patch.extend_from_slice(b"PATCH"); // Header

    let interval = rom_size / patch_count.max(1);
    for i in 0..patch_count {
        let offset = (i * interval) as u32;
        // Write offset (3 bytes BE)
        patch.push((offset >> 16) as u8);
        patch.push((offset >> 8) as u8);
        patch.push(offset as u8);
        // Write size (2 bytes BE)
        patch.push(0x00);
        patch.push(0x01);
        // Write data (1 byte)
        patch.push(0xFF);
    }

    patch.extend_from_slice(b"EOF"); // Footer

    // Add JSON metadata (EBP-specific)
    patch.extend_from_slice(b"{\"title\":\"Benchmark Patch\",\"author\":\"Test\",\"description\":\"Test patch for benchmarking\",\"version\":\"1.0\"}");

    patch
}

fn bench_ebp_apply(c: &mut Criterion) {
    let mut group = c.benchmark_group("ebp_apply");
    group.measurement_time(std::time::Duration::from_secs(15)); // Ensure no warnings for large files

    // Test from 1KB up to 16MB (EBP is IPS-based, so 24-bit addressing limit)
    for size in [
        1024,             // 1KB
        10 * 1024,        // 10KB
        100 * 1024,       // 100KB
        1024 * 1024,      // 1MB
        4 * 1024 * 1024,  // 4MB
        8 * 1024 * 1024,  // 8MB
        16 * 1024 * 1024, // 16MB (IPS max)
    ]
    .iter()
    {
        let patch = generate_test_patch(*size, 10);
        let original = vec![0u8; *size];

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                let mut rom = original.clone();
                EbpPatcher
                    .apply(black_box(&mut rom), black_box(&patch))
                    .unwrap();
            });
        });
    }

    group.finish();
}

fn bench_ebp_validate(c: &mut Criterion) {
    let mut group = c.benchmark_group("ebp_validate");

    // Test from 1KB up to 16MB (EBP is IPS-based, so 24-bit addressing limit)
    for size in [
        1024,             // 1KB
        10 * 1024,        // 10KB
        100 * 1024,       // 100KB
        1024 * 1024,      // 1MB
        4 * 1024 * 1024,  // 4MB
        8 * 1024 * 1024,  // 8MB
        16 * 1024 * 1024, // 16MB (IPS max)
    ]
    .iter()
    {
        let patch = generate_test_patch(*size, 10);

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                EbpPatcher::validate(black_box(&patch)).unwrap();
            });
        });
    }

    group.finish();
}

fn bench_ebp_metadata(c: &mut Criterion) {
    let mut group = c.benchmark_group("ebp_metadata");

    // Test from 1KB up to 16MB
    // Metadata extraction includes JSON parsing overhead
    for size in [
        1024,             // 1KB
        10 * 1024,        // 10KB
        100 * 1024,       // 100KB
        1024 * 1024,      // 1MB
        4 * 1024 * 1024,  // 4MB
        8 * 1024 * 1024,  // 8MB
        16 * 1024 * 1024, // 16MB
    ]
    .iter()
    {
        let patch = generate_test_patch(*size, 10);

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                EbpPatcher::metadata(black_box(&patch)).unwrap();
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_ebp_apply,
    bench_ebp_validate,
    bench_ebp_metadata
);
criterion_main!(benches);
