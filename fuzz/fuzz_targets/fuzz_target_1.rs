#![no_main]
use libfuzzer_sys::fuzz_target;

use bevy_fuzz::FuzzInput;

fuzz_target!(|data: Vec<FuzzInput>| {
    println!("Fuzz iter {}", data.len());
    let app = bevy_fuzz::get_app(data);
});
