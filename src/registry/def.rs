use crate::identifier::Identifier;
use crate::Registry;

pub trait Definition<'a> {
	/// The identifier of the definition.
	fn identifier(&self) -> &Identifier<'a>;
}
