use crate::i18n::Translatable;
use crate::identifier::Identifier;
use crate::Registry;
use crate::registry::def::Definition;
use serde::{Serialize, Deserialize};
use bevy::prelude::*;

pub struct ElementRegistry<'a>(pub Registry<'a, ElementDef<'a>>);

#[derive(Default)]
pub struct ElementDef<'a> {
	identifier: Identifier<'a>,
}

impl<'a> ElementDef<'a> {
	pub fn new(identifier: Identifier<'a>) -> Self {
		Self {
			identifier,
		}
	}
}

impl<'a> Definition<'a> for ElementDef<'a> {
	fn identifier(&self) -> &Identifier<'a> {
		&self.identifier
	}
}

impl<'a> Default for ElementRegistry<'a> {
	fn default() -> Self {
		ElementRegistry(Registry::default())
	}
}
