use super::WrapElementState;
use bevy::{
    input::mouse::{MouseButtonInput, MouseScrollUnit, MouseWheel},
    prelude::MouseButton,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WrapMouseButtonInput {
    pub button: WrapMouseButton,
    pub state: WrapElementState,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WrapMouseButton {
    Left,
    Right,
    Middle,
    Other(u16),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum WrapMouseScrollUnit {
    Line,
    Pixel,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WrapMouseWheel {
    pub unit: WrapMouseScrollUnit,
    pub x: f32,
    pub y: f32,
}

impl From<&WrapMouseButtonInput> for MouseButtonInput {
    fn from(inp: &WrapMouseButtonInput) -> Self {
        MouseButtonInput {
            button: (&inp.button).into(),
            state: (&inp.state).into(),
        }
    }
}

impl From<&MouseButtonInput> for WrapMouseButtonInput {
    fn from(inp: &MouseButtonInput) -> Self {
        WrapMouseButtonInput {
            button: (&inp.button).into(),
            state: (&inp.state).into(),
        }
    }
}

impl From<&WrapMouseButton> for MouseButton {
    fn from(inp: &WrapMouseButton) -> Self {
        match inp {
            WrapMouseButton::Left => MouseButton::Left,
            WrapMouseButton::Right => MouseButton::Right,
            WrapMouseButton::Middle => MouseButton::Middle,
            WrapMouseButton::Other(v) => MouseButton::Other(*v),
        }
    }
}

impl From<&MouseButton> for WrapMouseButton {
    fn from(inp: &MouseButton) -> Self {
        match inp {
            MouseButton::Left => WrapMouseButton::Left,
            MouseButton::Right => WrapMouseButton::Right,
            MouseButton::Middle => WrapMouseButton::Middle,
            MouseButton::Other(v) => WrapMouseButton::Other(*v),
        }
    }
}

impl From<&WrapMouseScrollUnit> for MouseScrollUnit {
    fn from(inp: &WrapMouseScrollUnit) -> Self {
        match inp {
            WrapMouseScrollUnit::Line => MouseScrollUnit::Line,
            WrapMouseScrollUnit::Pixel => MouseScrollUnit::Pixel,
        }
    }
}

impl From<&MouseScrollUnit> for WrapMouseScrollUnit {
    fn from(inp: &MouseScrollUnit) -> Self {
        match inp {
            MouseScrollUnit::Line => WrapMouseScrollUnit::Line,
            MouseScrollUnit::Pixel => WrapMouseScrollUnit::Pixel,
        }
    }
}

impl From<&WrapMouseWheel> for MouseWheel {
    fn from(inp: &WrapMouseWheel) -> Self {
        MouseWheel {
            unit: (&inp.unit).into(),
            x: inp.x,
            y: inp.y,
        }
    }
}

impl From<&MouseWheel> for WrapMouseWheel {
    fn from(inp: &MouseWheel) -> Self {
        WrapMouseWheel {
            unit: (&inp.unit).into(),
            x: inp.x,
            y: inp.y,
        }
    }
}
