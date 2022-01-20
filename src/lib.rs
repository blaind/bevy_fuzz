use std::time::Duration;

use arbitrary::Arbitrary;

#[derive(Debug, Clone, Arbitrary)]
pub enum FuzzInput {
    MouseButton(WrapMouseButtonInput),
    WrapKeyboardInput(WrapKeyboardInput),
    RunFrame,
}

#[derive(Debug, Clone, Arbitrary)]
pub enum WrapElementState {
    Pressed,
    Released,
}

#[derive(Debug, Clone, Arbitrary)]
pub struct WrapMouseButtonInput {
    pub button: WrapMouseButton,
    pub state: WrapElementState,
}

#[derive(Debug, Clone, Arbitrary)]
pub enum WrapMouseButton {
    Left,
    Right,
    Middle,
    Other(u16),
}

#[derive(Debug, Clone, Arbitrary)]
pub struct WrapKeyboardInput {
    pub scan_code: u32,
    pub key_code: Option<WrapKeyCode>,
    pub state: WrapElementState,
}

#[derive(Debug, Clone, Arbitrary)]
pub enum WrapKeyCode {
    /// The '1' key over the letters.
    Key1,
    /// The '2' key over the letters.
    Key2,
    /// The '3' key over the letters.
    Key3,
    /// The '4' key over the letters.
    Key4,
    /// The '5' key over the letters.
    Key5,
    /// The '6' key over the letters.
    Key6,
    /// The '7' key over the letters.
    Key7,
    /// The '8' key over the letters.
    Key8,
    /// The '9' key over the letters.
    Key9,
    /// The '0' key over the 'O' and 'P' keys.
    Key0,

    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    /// The Escape key, next to F1.
    Escape,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,

    /// Print Screen/SysRq.
    Snapshot,
    /// Scroll Lock.
    Scroll,
    /// Pause/Break key, next to Scroll lock.
    Pause,

    /// `Insert`, next to Backspace.
    Insert,
    Home,
    Delete,
    End,
    PageDown,
    PageUp,

    Left,
    Up,
    Right,
    Down,

    /// The Backspace key, right over Enter.
    Back,
    /// The Enter key.
    Return,
    /// The space bar.
    Space,

    /// The "Compose" key on Linux.
    Compose,

    Caret,

    Numlock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,

    AbntC1,
    AbntC2,
    NumpadAdd,
    Apostrophe,
    Apps,
    Asterisk,
    Plus,
    At,
    Ax,
    Backslash,
    Calculator,
    Capital,
    Colon,
    Comma,
    Convert,
    NumpadDecimal,
    NumpadDivide,
    Equals,
    Grave,
    Kana,
    Kanji,
    /// The left alt key. Maps to left option on Mac.
    LAlt,
    LBracket,
    LControl,
    LShift,
    /// The left Windows key. Maps to left Command on Mac.
    LWin,
    Mail,
    MediaSelect,
    MediaStop,
    Minus,
    NumpadMultiply,
    Mute,
    MyComputer,
    NavigateForward,  // also called "Prior"
    NavigateBackward, // also called "Next"
    NextTrack,
    NoConvert,
    NumpadComma,
    NumpadEnter,
    NumpadEquals,
    Oem102,
    Period,
    PlayPause,
    Power,
    PrevTrack,
    /// The right alt key. Maps to right option on Mac.
    RAlt,
    RBracket,
    RControl,
    RShift,
    /// The right Windows key. Maps to right Command on Mac.
    RWin,
    Semicolon,
    Slash,
    Sleep,
    Stop,
    NumpadSubtract,
    Sysrq,
    Tab,
    Underline,
    Unlabeled,
    VolumeDown,
    VolumeUp,
    Wake,
    WebBack,
    WebFavorites,
    WebForward,
    WebHome,
    WebRefresh,
    WebSearch,
    WebStop,
    Yen,
    Copy,
    Paste,
    Cut,
}

#[derive(Default)]
struct BevyFuzzPlugin {
    data: Vec<FuzzInput>,
}

impl BevyFuzzPlugin {
    pub fn new(data: Vec<FuzzInput>) -> Self {
        Self { data }
    }
}

struct FuzzData {
    last_index_id: Option<usize>,
    data: Vec<FuzzInput>,
}

impl Plugin for BevyFuzzPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FuzzData {
            last_index_id: None,
            data: self.data.clone(),
        });

        app.set_runner(move |mut app: App| {
            let mut app_exit_event_reader = ManualEventReader::<AppExit>::default();

            let mut tick = move |app: &mut App| -> Result<Option<Duration>, AppExit> {
                if let Some(app_exit_events) = app.world.get_resource_mut::<Events<AppExit>>() {
                    if let Some(exit) = app_exit_event_reader.iter(&app_exit_events).last() {
                        return Err(exit.clone());
                    }
                }

                let mut last_index_id = None;
                let mut fuzz_event_count = 0;

                {
                    let world = app.world.cell();
                    let mut data = world.get_resource_mut::<FuzzData>().unwrap();
                    fuzz_event_count = data.data.len();

                    let skip_count = data.last_index_id.map(|v| v + 1).unwrap_or(0);
                    //println!("ITER, skip {}", skip_count);
                    for (idx, event) in data.data.iter().skip(skip_count).enumerate() {
                        match event {
                            FuzzInput::MouseButton(wrap_mouse_button_input) => {
                                let mut mouse_button_input_events = world
                                    .get_resource_mut::<Events<MouseButtonInput>>()
                                    .unwrap();

                                mouse_button_input_events.send(MouseButtonInput {
                                    button: match wrap_mouse_button_input.button {
                                        WrapMouseButton::Left => MouseButton::Left,
                                        WrapMouseButton::Right => MouseButton::Right,
                                        WrapMouseButton::Middle => MouseButton::Middle,
                                        WrapMouseButton::Other(v) => MouseButton::Other(v),
                                    },
                                    state: match wrap_mouse_button_input.state {
                                        WrapElementState::Pressed => ElementState::Pressed,
                                        WrapElementState::Released => ElementState::Released,
                                    },
                                });
                            }
                            FuzzInput::WrapKeyboardInput(keyboard_input) => {
                                let mut keyboard_input_events =
                                    world.get_resource_mut::<Events<KeyboardInput>>().unwrap();

                                keyboard_input_events.send(KeyboardInput {
                                    scan_code: keyboard_input.scan_code,
                                    key_code: match &keyboard_input.key_code {
                                        Some(key_code) => Some(match key_code {
                                            WrapKeyCode::Key1 => KeyCode::Key1,
                                            WrapKeyCode::Key2 => KeyCode::Key2,
                                            WrapKeyCode::Key3 => KeyCode::Key3,
                                            WrapKeyCode::Key4 => KeyCode::Key4,
                                            WrapKeyCode::Key5 => KeyCode::Key5,
                                            WrapKeyCode::Key6 => KeyCode::Key6,
                                            WrapKeyCode::Key7 => KeyCode::Key7,
                                            WrapKeyCode::Key8 => KeyCode::Key8,
                                            WrapKeyCode::Key9 => KeyCode::Key9,
                                            WrapKeyCode::Key0 => KeyCode::Key0,
                                            WrapKeyCode::A => KeyCode::A,
                                            WrapKeyCode::B => KeyCode::B,
                                            WrapKeyCode::C => KeyCode::C,
                                            WrapKeyCode::D => KeyCode::D,
                                            WrapKeyCode::E => KeyCode::E,
                                            WrapKeyCode::F => KeyCode::F,
                                            WrapKeyCode::G => KeyCode::G,
                                            WrapKeyCode::H => KeyCode::H,
                                            WrapKeyCode::I => KeyCode::I,
                                            WrapKeyCode::J => KeyCode::J,
                                            WrapKeyCode::K => KeyCode::K,
                                            WrapKeyCode::L => KeyCode::L,
                                            WrapKeyCode::M => KeyCode::M,
                                            WrapKeyCode::N => KeyCode::N,
                                            WrapKeyCode::O => KeyCode::O,
                                            WrapKeyCode::P => KeyCode::P,
                                            WrapKeyCode::Q => KeyCode::Q,
                                            WrapKeyCode::R => KeyCode::R,
                                            WrapKeyCode::S => KeyCode::S,
                                            WrapKeyCode::T => KeyCode::T,
                                            WrapKeyCode::U => KeyCode::U,
                                            WrapKeyCode::V => KeyCode::V,
                                            WrapKeyCode::W => KeyCode::W,
                                            WrapKeyCode::X => KeyCode::X,
                                            WrapKeyCode::Y => KeyCode::Y,
                                            WrapKeyCode::Z => KeyCode::Z,
                                            WrapKeyCode::Escape => KeyCode::Escape,
                                            WrapKeyCode::F1 => KeyCode::F1,
                                            WrapKeyCode::F2 => KeyCode::F2,
                                            WrapKeyCode::F3 => KeyCode::F3,
                                            WrapKeyCode::F4 => KeyCode::F4,
                                            WrapKeyCode::F5 => KeyCode::F5,
                                            WrapKeyCode::F6 => KeyCode::F6,
                                            WrapKeyCode::F7 => KeyCode::F7,
                                            WrapKeyCode::F8 => KeyCode::F8,
                                            WrapKeyCode::F9 => KeyCode::F9,
                                            WrapKeyCode::F10 => KeyCode::F10,
                                            WrapKeyCode::F11 => KeyCode::F11,
                                            WrapKeyCode::F12 => KeyCode::F12,
                                            WrapKeyCode::F13 => KeyCode::F13,
                                            WrapKeyCode::F14 => KeyCode::F14,
                                            WrapKeyCode::F15 => KeyCode::F15,
                                            WrapKeyCode::F16 => KeyCode::F16,
                                            WrapKeyCode::F17 => KeyCode::F17,
                                            WrapKeyCode::F18 => KeyCode::F18,
                                            WrapKeyCode::F19 => KeyCode::F19,
                                            WrapKeyCode::F20 => KeyCode::F20,
                                            WrapKeyCode::F21 => KeyCode::F21,
                                            WrapKeyCode::F22 => KeyCode::F22,
                                            WrapKeyCode::F23 => KeyCode::F23,
                                            WrapKeyCode::F24 => KeyCode::F24,
                                            WrapKeyCode::Snapshot => KeyCode::Snapshot,
                                            WrapKeyCode::Scroll => KeyCode::Scroll,
                                            WrapKeyCode::Pause => KeyCode::Pause,
                                            WrapKeyCode::Insert => KeyCode::Insert,
                                            WrapKeyCode::Home => KeyCode::Home,
                                            WrapKeyCode::Delete => KeyCode::Delete,
                                            WrapKeyCode::End => KeyCode::End,
                                            WrapKeyCode::PageDown => KeyCode::PageDown,
                                            WrapKeyCode::PageUp => KeyCode::PageUp,
                                            WrapKeyCode::Left => KeyCode::Left,
                                            WrapKeyCode::Up => KeyCode::Up,
                                            WrapKeyCode::Right => KeyCode::Right,
                                            WrapKeyCode::Down => KeyCode::Down,
                                            WrapKeyCode::Back => KeyCode::Back,
                                            WrapKeyCode::Return => KeyCode::Return,
                                            WrapKeyCode::Space => KeyCode::Space,
                                            WrapKeyCode::Compose => KeyCode::Compose,
                                            WrapKeyCode::Caret => KeyCode::Caret,
                                            WrapKeyCode::Numlock => KeyCode::Numlock,
                                            WrapKeyCode::Numpad0 => KeyCode::Numpad0,
                                            WrapKeyCode::Numpad1 => KeyCode::Numpad1,
                                            WrapKeyCode::Numpad2 => KeyCode::Numpad2,
                                            WrapKeyCode::Numpad3 => KeyCode::Numpad3,
                                            WrapKeyCode::Numpad4 => KeyCode::Numpad4,
                                            WrapKeyCode::Numpad5 => KeyCode::Numpad5,
                                            WrapKeyCode::Numpad6 => KeyCode::Numpad6,
                                            WrapKeyCode::Numpad7 => KeyCode::Numpad7,
                                            WrapKeyCode::Numpad8 => KeyCode::Numpad8,
                                            WrapKeyCode::Numpad9 => KeyCode::Numpad9,
                                            WrapKeyCode::AbntC1 => KeyCode::AbntC1,
                                            WrapKeyCode::AbntC2 => KeyCode::AbntC2,
                                            WrapKeyCode::NumpadAdd => KeyCode::NumpadAdd,
                                            WrapKeyCode::Apostrophe => KeyCode::Apostrophe,
                                            WrapKeyCode::Apps => KeyCode::Apps,
                                            WrapKeyCode::Asterisk => KeyCode::Asterisk,
                                            WrapKeyCode::Plus => KeyCode::Plus,
                                            WrapKeyCode::At => KeyCode::At,
                                            WrapKeyCode::Ax => KeyCode::Ax,
                                            WrapKeyCode::Backslash => KeyCode::Backslash,
                                            WrapKeyCode::Calculator => KeyCode::Calculator,
                                            WrapKeyCode::Capital => KeyCode::Capital,
                                            WrapKeyCode::Colon => KeyCode::Colon,
                                            WrapKeyCode::Comma => KeyCode::Comma,
                                            WrapKeyCode::Convert => KeyCode::Convert,
                                            WrapKeyCode::NumpadDecimal => KeyCode::NumpadDecimal,
                                            WrapKeyCode::NumpadDivide => KeyCode::NumpadDivide,
                                            WrapKeyCode::Equals => KeyCode::Equals,
                                            WrapKeyCode::Grave => KeyCode::Grave,
                                            WrapKeyCode::Kana => KeyCode::Kana,
                                            WrapKeyCode::Kanji => KeyCode::Kanji,
                                            WrapKeyCode::LAlt => KeyCode::LAlt,
                                            WrapKeyCode::LBracket => KeyCode::LBracket,
                                            WrapKeyCode::LControl => KeyCode::LControl,
                                            WrapKeyCode::LShift => KeyCode::LShift,
                                            WrapKeyCode::LWin => KeyCode::LWin,
                                            WrapKeyCode::Mail => KeyCode::Mail,
                                            WrapKeyCode::MediaSelect => KeyCode::MediaSelect,
                                            WrapKeyCode::MediaStop => KeyCode::MediaStop,
                                            WrapKeyCode::Minus => KeyCode::Minus,
                                            WrapKeyCode::NumpadMultiply => KeyCode::NumpadMultiply,
                                            WrapKeyCode::Mute => KeyCode::Mute,
                                            WrapKeyCode::MyComputer => KeyCode::MyComputer,
                                            WrapKeyCode::NavigateForward => KeyCode::NavigateForward,
                                            WrapKeyCode::NavigateBackward => KeyCode::NavigateBackward,
                                            WrapKeyCode::NextTrack => KeyCode::NextTrack,
                                            WrapKeyCode::NoConvert => KeyCode::NoConvert,
                                            WrapKeyCode::NumpadComma => KeyCode::NumpadComma,
                                            WrapKeyCode::NumpadEnter => KeyCode::NumpadEnter,
                                            WrapKeyCode::NumpadEquals => KeyCode::NumpadEquals,
                                            WrapKeyCode::Oem102 => KeyCode::Oem102,
                                            WrapKeyCode::Period => KeyCode::Period,
                                            WrapKeyCode::PlayPause => KeyCode::PlayPause,
                                            WrapKeyCode::Power => KeyCode::Power,
                                            WrapKeyCode::PrevTrack => KeyCode::PrevTrack,
                                            WrapKeyCode::RAlt => KeyCode::RAlt,
                                            WrapKeyCode::RBracket => KeyCode::RBracket,
                                            WrapKeyCode::RControl => KeyCode::RControl,
                                            WrapKeyCode::RShift => KeyCode::RShift,
                                            WrapKeyCode::RWin => KeyCode::RWin,
                                            WrapKeyCode::Semicolon => KeyCode::Semicolon,
                                            WrapKeyCode::Slash => KeyCode::Slash,
                                            WrapKeyCode::Sleep => KeyCode::Sleep,
                                            WrapKeyCode::Stop => KeyCode::Stop,
                                            WrapKeyCode::NumpadSubtract => KeyCode::NumpadSubtract,
                                            WrapKeyCode::Sysrq => KeyCode::Sysrq,
                                            WrapKeyCode::Tab => KeyCode::Tab,
                                            WrapKeyCode::Underline => KeyCode::Underline,
                                            WrapKeyCode::Unlabeled => KeyCode::Unlabeled,
                                            WrapKeyCode::VolumeDown => KeyCode::VolumeDown,
                                            WrapKeyCode::VolumeUp => KeyCode::VolumeUp,
                                            WrapKeyCode::Wake => KeyCode::Wake,
                                            WrapKeyCode::WebBack => KeyCode::WebBack,
                                            WrapKeyCode::WebFavorites => KeyCode::WebFavorites,
                                            WrapKeyCode::WebForward => KeyCode::WebForward,
                                            WrapKeyCode::WebHome => KeyCode::WebHome,
                                            WrapKeyCode::WebRefresh => KeyCode::WebRefresh,
                                            WrapKeyCode::WebSearch => KeyCode::WebSearch,
                                            WrapKeyCode::WebStop => KeyCode::WebStop,
                                            WrapKeyCode::Yen => KeyCode::Yen,
                                            WrapKeyCode::Copy => KeyCode::Copy,
                                            WrapKeyCode::Paste => KeyCode::Paste,
                                            WrapKeyCode::Cut => KeyCode::Cut,
                                        }),
                                        None => None,
                                    },
                                    state: match keyboard_input.state {
                                        WrapElementState::Pressed => ElementState::Pressed,
                                        WrapElementState::Released => ElementState::Released,
                                    },
                                });
                            }
                            FuzzInput::RunFrame => {
                                //println!("\tset idx {}", idx + skip_count);
                                last_index_id = Some(idx + skip_count);
                                break;
                            }
                        }
                    }

                    if let Some(last_index_id) = last_index_id {
                        data.last_index_id = Some(last_index_id);
                    } else {
                        last_index_id = Some(data.data.len());
                        data.last_index_id = Some(data.data.len());
                    }
                }

                app.update();

                //println!("\tfuzz: {:?} {:?}", last_index_id, fuzz_event_count);
                if let Some(last_index_id) = last_index_id {
                    if last_index_id + 1 >= fuzz_event_count {
                        return Err(AppExit);
                    }
                } else {
                    return Err(AppExit);
                }

                if let Some(app_exit_events) = app.world.get_resource_mut::<Events<AppExit>>() {
                    if let Some(exit) = app_exit_event_reader.iter(&app_exit_events).last() {
                        return Err(exit.clone());
                    }
                }

                Ok(None)
            };

            while let Ok(delay) = tick(&mut app) {
                if let Some(delay) = delay {
                    std::thread::sleep(delay);
                }
            }
        });
    }
}

use bevy::{
    app::{AppExit, Events, ManualEventReader},
    input::{keyboard::KeyboardInput, mouse::MouseButtonInput, ElementState},
    prelude::*,
};

pub fn get_app(data: Vec<FuzzInput>) {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(bevy::core::CorePlugin::default())
        .add_plugin(bevy::input::InputPlugin::default())
        .add_plugin(BevyFuzzPlugin::new(data))
        .add_system(keyboard_input_system)
        .run();
}

/// This system prints 'A' key state
fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::A) {
        info!("'A' currently pressed");
    }

    if keyboard_input.just_pressed(KeyCode::A) {
        info!("'A' just pressed");
    }

    if keyboard_input.just_released(KeyCode::A) {
        info!("'A' just released");
    }

    if keyboard_input.just_pressed(KeyCode::Z) {
        panic!();
    }
}
