use std::collections::HashSet;
use crate::Identifier;

/// A palette. This maps identifiers to indexes which can be used to store those identifiers more
/// efficiently and less repetitively. This is especially useful when dealing with something
/// inherently repetitive like blocks in a chunk.
pub struct Palette<'a, T> {
	index: HashSet<T>,
}

impl<'a, T> Palette<'a, T> {
	fn new() -> Self {
		Palette {
			index: HashSet::new(),
		}
	}
	
	fn insert(&mut self, x: T) {
		self.index.insert(x);
	}
	
	fn remove(&mut self, x: &T) {
		self.index.remove(x);
	}
}

/// Compresses chunks of block data via palettes. Useful for sending chunk data via network and
/// saving it.
pub struct PalettedBlockContainer {}
