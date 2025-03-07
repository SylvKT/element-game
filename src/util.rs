use bincode::{DefaultOptions, Error, Options};

pub fn serialize<T>(ser: &T) -> Result<Vec<u8>, Error>
	where
		T: serde::ser::Serialize,
{
	DefaultOptions::new()
		.with_fixint_encoding()
		.with_big_endian()
		.reject_trailing_bytes()
		.serialize(ser)
}

pub fn serialize_trailing<T>(ser: &T) -> Result<Vec<u8>, Error>
	where
		T: serde::ser::Serialize,
{
	DefaultOptions::new()
		.with_fixint_encoding()
		.with_big_endian()
		.allow_trailing_bytes()
		.serialize(ser)
}

pub fn deserialize<'a, T>(bytes: &'a [u8]) -> Result<T, Error>
	where
		T: serde::de::Deserialize<'a>,
{
	DefaultOptions::new()
		.with_fixint_encoding()
		.with_big_endian()
		.reject_trailing_bytes()
		.deserialize(bytes)
}

pub fn deserialize_trailing<'a, T>(bytes: &'a [u8]) -> Result<T, Error>
	where
		T: serde::de::Deserialize<'a>,
{
	DefaultOptions::new()
		.with_fixint_encoding()
		.with_big_endian()
		.allow_trailing_bytes()
		.deserialize(bytes)
}
