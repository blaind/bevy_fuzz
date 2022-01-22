use bevy::input::ElementState;
use serde::{Deserialize, Serialize};

pub mod keyboard;
pub mod mouse;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WrapElementState {
    Pressed,
    Released,
}

impl From<&WrapElementState> for ElementState {
    fn from(inp: &WrapElementState) -> Self {
        match inp {
            WrapElementState::Pressed => ElementState::Pressed,
            WrapElementState::Released => ElementState::Released,
        }
    }
}

impl From<&ElementState> for WrapElementState {
    fn from(inp: &ElementState) -> Self {
        match inp {
            ElementState::Pressed => WrapElementState::Pressed,
            ElementState::Released => WrapElementState::Released,
        }
    }
}
