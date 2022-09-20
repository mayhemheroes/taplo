#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    _ = taplo::parser::parse(data);
});
