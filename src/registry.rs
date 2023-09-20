pub mod element;
pub mod def;

use std::collections::HashMap;
use std::hash::{BuildHasher, Hash, Hasher};

use crate::identifier::Identifier;
use crate::registry::def::Definition;

/// A collection of items mapped to identifiers.
#[derive(Default)]
pub struct Registry<'a, T> where T : Definition<'a> {
	items: HashMap<Identifier<'a>, T>,
}

impl<'a, T> Registry<'a, T> where T : Definition<'a> {
	pub fn new() -> Self {
		Self {
			items: HashMap::new(),
		}
	}

	/// Registers a `T` in the `Registry<T>` with its given `Identifier`.
	pub fn register(&mut self, item: T) {
		self.items.insert(item.identifier().clone(), item);
	}

	/// Returns a `&T` mapped to the given `Identifier`.
	pub fn get(&self, id: &Identifier<'a>) -> Option<&T> {
		self.items.get(id)
	}
}
