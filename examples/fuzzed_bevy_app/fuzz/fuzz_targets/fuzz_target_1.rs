#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let my_plugin = fuzzed_bevy_app::MyAppPlugin::default();
    bevy_fuzz::fuzz_bootstrap(my_plugin, data);
});
