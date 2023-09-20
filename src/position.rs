use bevy::prelude::*;
use serde::*;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Component)]
pub struct BlockPos(pub i32, pub i32, pub i32);

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Component)]
pub struct Position(pub f32, pub f32, pub f32);
