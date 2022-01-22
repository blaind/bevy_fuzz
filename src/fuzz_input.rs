use crate::input::{keyboard, mouse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FuzzInput {
    MouseButton(mouse::WrapMouseButtonInput),
    KeyboardInput(keyboard::WrapKeyboardInput),
    MouseWheel(mouse::WrapMouseWheel),
    MouseMotion(crate::math::WrapVec2),
    CursorMoved(crate::window::event::WrapCursorMoved),
    WindowResized(crate::window::event::WrapWindowResized),
    RunFrame,
}
