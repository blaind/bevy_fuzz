use bevy::{
    app::{AppExit, Events, ManualEventReader},
    input::{
        keyboard::KeyboardInput,
        mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    },
    prelude::*,
    window::WindowResized,
};

use crate::data::FuzzData;
use crate::fuzz_input::FuzzInput;

const CORE_STAGES: &[CoreStage] = &[
    CoreStage::First,
    CoreStage::PreUpdate,
    CoreStage::Update,
    CoreStage::PostUpdate,
    CoreStage::Last,
];

pub fn fuzz_runner(app: &mut App) {
    // reset the world (TODO: first run is not necessary)
    reset_app_world(app);

    let mut app_exit_event_reader = ManualEventReader::<AppExit>::default();

    let mut tick_no = 0;
    while let Ok(_) = tick(app, tick_no, &mut app_exit_event_reader) {
        tick_no += 1;
    }
}

/// Tick (update) a frame
fn tick(
    app: &mut App,
    _tick_no: usize,
    app_exit_event_reader: &mut ManualEventReader<AppExit>,
) -> Result<(), AppExit> {
    // Feed the fuzzing input events into the world
    feed_fuzz_events(app);

    // Run only core stages
    for stage in CORE_STAGES {
        app.schedule
            .stage(stage.clone(), |stage: &mut SystemStage| {
                stage.run(&mut app.world);
                stage
            });
    }

    // return `AppExit` if there is no more fuzzing input to feed
    if app.world.get_resource::<FuzzData>().unwrap().is_finished() {
        return Err(AppExit);
    }

    if let Some(app_exit_events) = app.world.get_resource_mut::<Events<AppExit>>() {
        if let Some(exit) = app_exit_event_reader.iter(&app_exit_events).last() {
            return Err(exit.clone());
        }
    }

    Ok(())
}

/// Should reset the App to the initial state
fn reset_app_world(app: &mut App) {
    // TODO: is this really needed?
    app.world.clear_entities();

    app.schedule
        .stage(CoreStage::First, |stage: &mut SystemStage| {
            stage.run(&mut app.world);
            stage
        });

    app.schedule
        .stage(CoreStage::Startup, |schedule: &mut Schedule| {
            schedule.run_once(&mut app.world);
            schedule
        });

    for stage in CORE_STAGES {
        app.schedule
            .stage(stage.clone(), |stage: &mut SystemStage| {
                stage.run(&mut app.world);
                stage
            });
    }
}

/// Will send the fuzz input events to world until `FuzzInput::RunFrame` is detected
fn feed_fuzz_events(app: &mut App) {
    let world = app.world.cell();
    let mut data = world.get_resource_mut::<FuzzData>().unwrap();

    // event senders
    let mut mouse_button_input_events =
        world.get_resource_mut::<Events<MouseButtonInput>>().expect(
            "Missing MouseButtonInput events (provided by bevy::input::InputPlugin) from the App",
        );
    let mut mouse_wheel_input_events = world
        .get_resource_mut::<Events<MouseWheel>>()
        .expect("Missing MouseWheel events (provided by bevy::input::InputPlugin) from the App");
    let mut mouse_motion_events = world
        .get_resource_mut::<Events<MouseMotion>>()
        .expect("Missing MouseMotion events (provided by bevy::input::InputPlugin) from the App");
    let mut keyboard_input_events = world
        .get_resource_mut::<Events<KeyboardInput>>()
        .expect("Missing KeyboardInput events (provided by bevy::input::InputPlugin) from the App");
    let mut cursor_moved_events = world
        .get_resource_mut::<Events<CursorMoved>>()
        .expect("Missing CursorMoved events (provided by bevy::window::WindowPlugin) from the App");
    let mut window_resized_events = world.get_resource_mut::<Events<WindowResized>>().expect(
        "Missing WindowResized events (provided by bevy::window::WindowPlugin) from the App",
    );

    // loop
    let mut break_at_idx = None;
    for (idx, event) in data.iter_next().enumerate() {
        match event {
            FuzzInput::MouseButton(wrap_mouse_button_input) => {
                mouse_button_input_events.send(wrap_mouse_button_input.into());
            }
            FuzzInput::KeyboardInput(keyboard_input) => {
                keyboard_input_events.send(KeyboardInput {
                    scan_code: 0, // FIXME keyboard_input.scan_code,
                    key_code: match &keyboard_input.key_code {
                        Some(key_code) => Some(key_code.into()),
                        None => None,
                    },
                    state: (&keyboard_input.state).into(),
                });
            }
            FuzzInput::RunFrame => {
                break_at_idx = Some(idx);
                break;
            }
            FuzzInput::MouseWheel(mouse_wheel) => mouse_wheel_input_events.send(mouse_wheel.into()),
            FuzzInput::MouseMotion(delta) => {
                mouse_motion_events.send(MouseMotion {
                    delta: delta.into(),
                });
            }
            FuzzInput::CursorMoved(cursor_moved) => cursor_moved_events.send(cursor_moved.into()),
            FuzzInput::WindowResized(window_resized) => {
                window_resized_events.send(window_resized.into())
            }
        }
    }

    data.set_last_idx(break_at_idx);
}
