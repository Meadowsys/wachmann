//! structs and things for messages sent by the client to the server

use serde::Serialize;
use std::num::NonZeroU64;

#[derive(Serialize, Debug)]
#[serde(tag = "message")]
#[serde(rename_all = "snake_case")]
pub enum ClientMessage {
	PutTestData {
		data: String
	},
	GetTestData {
		id: String
	},
	SaveMessage {
		#[serde(with = "super::string")]
		id: NonZeroU64,
		#[serde(with = "super::string")]
		channel_id: NonZeroU64,
		#[serde(with = "super::string")]
		author_id: NonZeroU64,
		content: String,
		attachment_urls: Vec<String>
	}
}
