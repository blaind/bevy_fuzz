use bevy::window::WindowId;
use serde::{Deserialize, Serialize};

pub mod event;

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct WrapWindowId([u8; 16]);

impl std::fmt::Debug for WrapWindowId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WrapWindowId(primary)")
    }
}

impl From<&WrapWindowId> for WindowId {
    fn from(inp: &WrapWindowId) -> Self {
        WindowId::primary() // FIXME
    }
}

impl From<&WindowId> for WrapWindowId {
    fn from(inp: &WindowId) -> Self {
        if !inp.is_primary() {
            panic!("Only primary window type is supported currently");
        }

        WrapWindowId([0u8; 16]) // FIXME
    }
}
