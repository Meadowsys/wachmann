pub mod client_messages;
pub mod server_messages;
pub mod config;


// copy pasted from twilight_model/src/user/mod.rs
#[allow(unused)]
mod discriminator {
	use twilight_bot_utils::prelude::*;
	use twilight_model::user::DiscriminatorDisplay;

	use serde::{
		de::{Deserializer, Error as DeError, Visitor},
		ser::Serializer,
	};
	use std::{
		convert::TryInto,
		fmt::{Formatter, Result as FmtResult},
	};

	struct DiscriminatorVisitor;

	impl<'de> Visitor<'de> for DiscriminatorVisitor {
		type Value = u16;

		fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
			f.write_str("string or integer discriminator")
		}

		fn visit_u64<E: DeError>(self, value: u64) -> Result<Self::Value, E> {
			value.try_into().map_err(DeError::custom)
		}

		fn visit_str<E: DeError>(self, value: &str) -> Result<Self::Value, E> {
			value.parse().map_err(DeError::custom)
		}
	}

	pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u16, D::Error> {
		deserializer.deserialize_any(DiscriminatorVisitor)
	}

	// Allow this lint because taking a reference is required by serde.
	#[allow(clippy::trivially_copy_pass_by_ref)]
	pub fn serialize<S: Serializer>(value: &u16, serializer: S) -> Result<S::Ok, S::Error> {
		serializer.collect_str(&DiscriminatorDisplay::new(*value))
	}
}
