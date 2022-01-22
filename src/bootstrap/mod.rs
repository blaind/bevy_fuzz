use bevy::prelude::*;

mod bin;
mod fuzz;
pub use bin::bin_bootstrap;
pub use fuzz::fuzz_bootstrap;

use crate::{
    prelude::{FuzzData, FuzzInput},
    EventOutputPlugin,
};

pub trait FuzzTarget: Plugin {
    fn add_headless_plugins(&mut self, app: &mut App) {
        app.add_plugins(MinimalPlugins)
            .add_plugin(bevy::core::CorePlugin::default())
            .add_plugin(bevy::input::InputPlugin::default())
            .add_plugin(bevy::window::WindowPlugin::default());
    }

    fn add_gui_plugins(&mut self, app: &mut App) {
        app.add_plugins(DefaultPlugins);
    }

    /// Internal: do not implement
    fn enable_gui_mode(&mut self, app: &mut App) {
        self.add_gui_plugins(app);
        self.build(app);
    }

    /// Internal: do not implement
    fn enable_recording_mode(&mut self, app: &mut App) {
        self.add_gui_plugins(app);
        app.add_plugin(EventOutputPlugin::default());
        self.build(app);
    }

    /// Internal: do not implement
    fn enable_fuzzing_mode(&mut self, app: &mut App) {
        self.add_headless_plugins(app);
        self.build(app);
    }

    /// Internal: do not implement
    fn enable_apply_mode(&mut self, app: &mut App, data: Vec<FuzzInput>) {
        self.add_headless_plugins(app);
        app.insert_resource(FuzzData::new(data));
        self.build(app);
    }
}
