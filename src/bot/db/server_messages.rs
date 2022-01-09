//! structs and things for messages sent by the server to the client

use serde::Deserialize;
use std::num::NonZeroU64;

#[derive(Deserialize, Debug)]
#[serde(tag = "message")]
#[serde(rename_all = "snake_case")]
pub enum ServerMessage {
	Ready {},
	Ok {},
	Error {
		error: String
	},
	Message {
		#[serde(with = "super::string")]
		id: NonZeroU64,
		#[serde(with = "super::string")]
		channel_id: NonZeroU64,
		#[serde(with = "super::string")]
		author_id: NonZeroU64,
		content: String,
		attachment_urls: Vec<String>
	},
	NoMessage {}
}
