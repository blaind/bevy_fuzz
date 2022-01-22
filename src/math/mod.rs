use bevy::math::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WrapVec2 {
    pub x: f32,
    pub y: f32,
}

impl From<&WrapVec2> for Vec2 {
    fn from(inp: &WrapVec2) -> Self {
        Vec2::new(inp.x, inp.y)
    }
}

impl From<&Vec2> for WrapVec2 {
    fn from(inp: &Vec2) -> Self {
        WrapVec2 {
            x: inp.x,
            y: inp.y
        }
    }
}

