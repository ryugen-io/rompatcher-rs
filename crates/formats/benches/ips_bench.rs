use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use rom_patcher_core::PatchFormat;
use rom_patcher_formats::ips::IpsPatcher;

/// Generate a test IPS patch that writes 0xFF at 10 evenly-spaced offsets
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
    patch
}

fn bench_ips_apply(c: &mut Criterion) {
    let mut group = c.benchmark_group("ips_apply");

    for size in [1024, 10 * 1024, 100 * 1024, 1024 * 1024].iter() {
        let patch = generate_test_patch(*size, 10);
        let original = vec![0u8; *size];

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                let mut rom = original.clone();
                IpsPatcher
                    .apply(black_box(&mut rom), black_box(&patch))
                    .unwrap();
            });
        });
    }

    group.finish();
}

fn bench_ips_validate(c: &mut Criterion) {
    let mut group = c.benchmark_group("ips_validate");

    for size in [1024, 10 * 1024, 100 * 1024, 1024 * 1024].iter() {
        let patch = generate_test_patch(*size, 10);

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                IpsPatcher::validate(black_box(&patch)).unwrap();
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_ips_apply, bench_ips_validate);
criterion_main!(benches);
