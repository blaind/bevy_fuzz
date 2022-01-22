# bevy_fuzz &emsp; [![Build Status]][actions] [![Latest Version]][crates.io] [![Docs Version]][docs]

[build status]: https://img.shields.io/github/workflow/status/blaind/bevy_fuzz/test
[actions]: https://github.com/blaind/bevy_fuzz/actions?query=branch%3Amain
[latest version]: https://img.shields.io/crates/v/bevy_fuzz.svg
[crates.io]: https://crates.io/crates/bevy_fuzz
[docs version]: https://docs.rs/bevy_fuzz/badge.svg
[docs]: https://docs.rs/bevy_fuzz

Experimental high-performance code coverage-based fuzz-testing for bevy systems, emulating user UI interaction. This plugin works by constructing and sending random input events to the application

The purpose is to find combinations of user interactions that produce crashes. The package uses
[cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz) as a Rust frontend, which itself uses
[libFuzzer](https://llvm.org/docs/LibFuzzer.html) to run the fuzzing.

The fuzzing is smart in a sense, that it tries to find inputs that cover as much code space as possible. Currently captured/fuzzed inputs:

- bevy_input / mouse / MouseButtonInput
- bevy_input / mouse / MouseWheel
- bevy_input / mouse / MouseMotion
- bevy_input / keyboard / KeyboardInput
- bevy_window / CursorMoved
- bevy_window / WindowResized

### Warnings / Please note

- The produced binary files are not currently compatible between various bevy_fuzz (and bevy) versions
- Only built-in `CoreStage`'s are run currently. Subapps (and render graph) are ignored
- Public API (e.g. function & struct names) will most probably change in the future
- This is still a "tech preview", and feasibility of UI fuzzing for actually finding bugs remains to be seen
- Currently tested only on Linux

## Quick start

### Running an integrated example

Install the required tooling (`cargo-fuzz`)

    cargo install cargo-fuzz

Clone the repository and go to sample directory

    git clone https://github.com/blaind/bevy_fuzz.git
    cd bevy_fuzz/examples/fuzzed_bevy_app

Run the app in a input-recording mode. It will show a grey window, try pressing various keys, including key A. You should see the output of key A in the console.

    cargo run --features fuzz -- record

This will produce a file called `input-recording.bin`. Optional: you can view the recording by:

    cargo run --features fuzz -- view input-recording.bin

Copy the file to fuzzing corpus directory:

    mkdir -p fuzz/corpus/fuzz_target_1/
    cp input-recording.bin fuzz/corpus/fuzz_target_1/

Run the fuzzer. For now, the `-s none` (sanitizer = none) is an important build toggle. This will recompile the app using LLVM instructions. The fuzzing -compilation is slower than standard compilation.

    cargo fuzz run -s none fuzz_target_1 -- -detect_leaks=0 -rss_limit_mb=8192

Eventually, this should crash as the fuzzer finds a keypress Z (which intentionally panics). The output should be similar to:

    WARNING: Failed to find function "__sanitizer_acquire_crash_state".
    WARNING: Failed to find function "__sanitizer_print_stack_trace".
    WARNING: Failed to find function "__sanitizer_set_death_callback".
    INFO: Running with entropic power schedule (0xFF, 100).
    INFO: Seed: 1184633377
    INFO: Loaded 1 modules   (979081 inline 8-bit counters): 979081 [0x55a818b8d889, 0x55a818c7c912),
    INFO: Loaded 1 PC tables (979081 PCs): 979081 [0x55a818c7c918,0x55a819b6d1a8),
    INFO:      848 files found in bevy_fuzz/examples/fuzzed_bevy_app/fuzz/corpus/fuzz_target_1
    INFO: -max_len is not provided; libFuzzer will not generate inputs larger than 4096 bytes
    INFO: seed corpus: files: 848 min: 32b max: 1346b total: 815506b rss: 68Mb
    #256	pulse  cov: 3603 ft: 9014 corp: 216/89Kb exec/s: 128 rss: 74Mb
    'A' currently pressed
    'A' just pressed
    (presses repeated)
    thread '<unnamed>' panicked at ''Z' pressed - causes panic!', bevy_fuzz/examples/fuzzed_bevy_app/src/lib.rs:48:9
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    ==3696259== ERROR: libFuzzer: deadly signal
    NOTE: libFuzzer has rudimentary signal handlers.
        Combine libFuzzer with AddressSanitizer or similar for better crash reports.
    SUMMARY: libFuzzer: deadly signal
    MS: 0 ; base unit: 0000000000000000000000000000000000000000

Output will also print the binary sequence that caused the crash. You can rerun the crashing artifact again either with

A. the provided main-wrapper (faster, preferred)

    cargo run --features fuzz apply fuzz/artifacts/fuzz_target_1/crash-[filename]

B. cargo fuzz (slower)

    cargo fuzz run --sanitizer=none fuzz_target_1 fuzz/artifacts/fuzz_target_1/crash-[filename]

**NOTE! If the above commands do not reproduce the crash, the run is not deterministic**. This is
currently hard to debug, please file an issue. One cause can be that you have `.insert_resource`'s
in your `Plugin` builder `fn build(&self, app: &mut App)`. These resources can not be reset
for each run currently - you should move them to a startup system.

Often it's also good to try to minimize the crash

    cargo fuzz tmin --sanitizer=none fuzz_target_1 fuzz/artifacts/fuzz_target_1/crash-[filename]

### Integrating to own app

Instructions coming. For now, see the [examples/fuzzed_bevy_app](examples/fuzzed_bevy_app) example.

## Other resources

https://rust-fuzz.github.io/book/introduction.html

## License

Licensed under either of

- <a href="LICENSE-APACHE">Apache License, Version 2.0</a> or
- <a href="LICENSE-MIT">MIT license</a>

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the software by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
