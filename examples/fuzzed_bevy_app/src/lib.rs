use bevy::prelude::*;

/// This is your app logic
#[derive(Default)]
pub struct MyAppPlugin;

impl Plugin for MyAppPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(keyboard_input_system);
    }
}

/// Implements app building for usage in fuzzing
#[cfg(feature = "fuzz")]
impl bevy_fuzz::FuzzTarget for MyAppPlugin {
    /// Add plugins that are needed when running in a GUI mode
    /// (if you use the exact same set as below, no need to implement the fn, they're in trait)
    fn add_gui_plugins(&mut self, app: &mut App) {
        app.add_plugins(DefaultPlugins);
    }

    /// Add a minimal set of plugins for headless mode
    ///
    /// Usually this is a bit of trial and error
    /// (if you use the exact same set as below, no need to implement the fn, they're in trait)
    fn add_headless_plugins(&mut self, app: &mut App) {
        app.add_plugins(MinimalPlugins)
            .add_plugin(bevy::core::CorePlugin::default())
            .add_plugin(bevy::input::InputPlugin::default())
            .add_plugin(bevy::window::WindowPlugin::default());
    }
}

fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::A) {
        println!("'A' currently pressed");
    }

    if keyboard_input.just_pressed(KeyCode::A) {
        println!("'A' just pressed");
    }

    if keyboard_input.just_released(KeyCode::A) {
        println!("'A' just released");
    }

    if keyboard_input.just_pressed(KeyCode::Z) {
        panic!("'Z' pressed - causes panic!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_plugin() {
        assert_eq!(Some(MyAppPlugin::default()).is_some(), true);
    }

    #[test]
    #[cfg(feature = "fuzz")]
    fn test_fuzz_plugin() {
        use bevy_fuzz::FuzzTarget;
        let mut app = App::new();
        let mut my_app_plugin = MyAppPlugin::default();
        my_app_plugin.add_headless_plugins(&mut app);
        assert_eq!(Some(my_app_plugin).is_some(), true);
    }
}
