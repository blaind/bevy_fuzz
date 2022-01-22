use bevy_fuzz::prelude::*;
use fuzzed_bevy_app::MyAppPlugin;

fn main() {
    // Construct only the Plugin, no app initialization yet!
    let my_plugin = MyAppPlugin::default();

    // Bootstrap the plugin/app, will build the app and run in CLI mode
    bin_bootstrap(my_plugin, std::env::args());
}
