use bevy::window::{CursorMoved, WindowResized};
use serde::{Deserialize, Serialize};

use crate::math::WrapVec2;
use super::WrapWindowId;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WrapWindowResized {
    pub id: WrapWindowId,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WrapCursorMoved {
    pub id: WrapWindowId,
    pub position: WrapVec2,
}

impl From<&WrapWindowResized> for WindowResized {
    fn from(inp: &WrapWindowResized) -> Self {
        WindowResized {
            id: (&inp.id).into(),
            width: inp.width,
            height: inp.height
        }
    }
}

impl From<&WindowResized> for WrapWindowResized {
    fn from(inp: &WindowResized) -> Self {
        WrapWindowResized {
            id: (&inp.id).into(),
            width: inp.width,
            height: inp.height
        }
    }
}


impl From<&WrapCursorMoved> for CursorMoved {
    fn from(inp: &WrapCursorMoved) -> Self {
        CursorMoved {
            id: (&inp.id).into(),
            position: (&inp.position).into(),
        }
    }
}

impl From<&CursorMoved> for WrapCursorMoved {
    fn from(inp: &CursorMoved) -> Self {
        WrapCursorMoved {
            id: (&inp.id).into(),
            position: (&inp.position).into(),
        }
    }
}

