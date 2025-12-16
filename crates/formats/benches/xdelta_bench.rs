use divan::Bencher;
use rom_patcher_core::PatchFormat;
use rom_patcher_formats::xdelta::XdeltaPatcher;

fn main() {
    divan::main();
}

// We use the existing test file for benchmarking
const PATCH_BYTES: &[u8] = include_bytes!("../../../test_files/xdelta/patch.xdelta");

#[divan::bench]
fn xdelta_validate(bencher: Bencher) {
    bencher.bench(|| {
        XdeltaPatcher::validate(divan::black_box(PATCH_BYTES)).unwrap();
    });
}

// We can attempt to apply, but it might fail due to checksum mismatch with dummy ROM.
// We just want to measure the overhead of parsing up to that failure point or full apply if valid.
#[divan::bench]
fn xdelta_apply_dummy(bencher: Bencher) {
    let rom = vec![0u8; 1024 * 1024]; // 1MB dummy ROM

    bencher.bench_local(|| {
        let mut rom_clone = rom.clone();
        // Ignore result as we expect failure or partial success depending on patch content
        let _ = XdeltaPatcher.apply(divan::black_box(&mut rom_clone), divan::black_box(PATCH_BYTES));
    });
}
