use std::collections::HashMap;
use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use crate::networking::protocol::ClientId;
use crate::position::*;
use crate::raw_id::RawId;

pub struct GameWorlds(pub HashMap<String, GameWorldId>, pub HashMap<GameWorldId, GameWorld>);

#[derive(Debug, Clone, Serialize, Deserialize, Component)]
pub struct GameWorld {
	id: GameWorldId,
	name: &'static str,
	players: Vec<ClientId>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GameWorldId(pub u64);
