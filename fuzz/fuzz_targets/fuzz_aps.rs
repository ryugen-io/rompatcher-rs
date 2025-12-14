#![no_main]
use libfuzzer_sys::fuzz_target;
use rom_patcher_formats::aps::ApsPatcher;
use rom_patcher_core::PatchFormat;

fuzz_target!(|data: &[u8]| {
    // APS checks strict signatures in keys, so many random inputs fail fast.
    let _ = ApsPatcher::validate(data);
    let _ = ApsPatcher::metadata(data);

    let mut rom = vec![0u8; 1024];
    let patcher = ApsPatcher;
    let _ = patcher.apply(&mut rom, data);
});
