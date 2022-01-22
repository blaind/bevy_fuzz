use std::{fs::File, io::Write};

use bevy::{
    input::{
        keyboard::KeyboardInput,
        mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    },
    prelude::*,
    window::WindowResized,
};

use crate::{math::WrapVec2, prelude::FuzzInput};

#[derive(Default)]
pub struct EventOutputPlugin;

impl Plugin for EventOutputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EventOutput {
            buffer: vec![0u8; 1024 * 1024],
            file: File::create("fuzz-recording.bin").unwrap(),
        })
        .add_system(output_input_events_system);
    }
}

pub struct EventOutput {
    buffer: Vec<u8>,
    file: File,
}

impl EventOutput {
    pub fn write_input_actions(&mut self, input_actions: &[FuzzInput]) -> Result<(), ()> {
        for action in input_actions {
            let cobs_encoded_data =
                postcard::to_slice_cobs(action, self.buffer.as_mut_slice()).unwrap();

            self.file.write(cobs_encoded_data).unwrap(); // TODO use bufwriter for performance?
        }

        Ok(())
    }
}

pub fn output_input_events_system(
    mut window_resized_events: EventReader<WindowResized>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut event_output: ResMut<EventOutput>,
) {
    let mut f_inp = Vec::new();

    f_inp.extend(
        window_resized_events
            .iter()
            .map(|v| FuzzInput::WindowResized(v.into())),
    );
    f_inp.extend(
        mouse_button_input_events
            .iter()
            .map(|v| FuzzInput::MouseButton(v.into())),
    );
    f_inp.extend(
        mouse_wheel_events
            .iter()
            .map(|v| FuzzInput::MouseWheel(v.into())),
    );
    f_inp.extend(
        keyboard_input_events
            .iter()
            .map(|v| FuzzInput::KeyboardInput(v.into())),
    );
    f_inp.extend(mouse_motion_events.iter().map(|v| {
        FuzzInput::MouseMotion(WrapVec2 {
            x: v.delta.x,
            y: v.delta.y,
        })
    }));
    f_inp.extend(
        cursor_moved_events
            .iter()
            .map(|v| FuzzInput::CursorMoved(v.into())),
    );

    f_inp.extend([FuzzInput::RunFrame]);

    event_output.write_input_actions(&f_inp).unwrap();
}

pub fn parse_commands(mut input: Vec<u8>) -> Result<Vec<FuzzInput>, ()> {
    let mut buf = vec![0u8; 1024];

    let mut offset = 0;
    let mut fuzz_inputs = Vec::new();
    loop {
        let input_slice = &mut input[offset..];
        let mut cd = postcard_cobs::CobsDecoder::new(&mut buf);

        match cd.push(&input_slice) {
            Ok(v) => match v {
                Some((_, used_data)) => {
                    match postcard::from_bytes_cobs::<FuzzInput>(&mut input_slice[..used_data]) {
                        Ok(v) => fuzz_inputs.push(v),
                        Err(_) => {
                            return Err(());
                        }
                    }

                    offset += used_data;
                }
                None => break,
            },
            Err(_) => break,
        }
    }

    Ok(fuzz_inputs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_commands() {
        let data = vec![
            2, 3, 1, 3, 128, 192, 1, 3, 128, 63, 0, 2, 3, 1, 3, 208, 65, 1, 3, 200, 65, 0,
        ];

        let commands = parse_commands(data).unwrap();

        assert_eq!(
            &commands,
            &[
                FuzzInput::MouseMotion(WrapVec2 { x: -4.0, y: 1.0 }),
                FuzzInput::MouseMotion(WrapVec2 { x: 26.0, y: 25.0 })
            ]
        );
    }
}
