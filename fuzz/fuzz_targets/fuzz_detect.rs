#![no_main]
use libfuzzer_sys::fuzz_target;
use rom_patcher_formats::detect_format;

fuzz_target!(|data: &[u8]| {
    let _ = detect_format(data);
});
