//! structs and things for messages sent by the client to the server

// https://github.com/serde-rs/serde/issues/760
// workaround for now: https://github.com/serde-rs/serde/issues/760#issuecomment-499570311

use serde::Serialize;
use std::num::NonZeroU64;

pub trait ClientMessage: Serialize {}

#[derive(Serialize, Debug)]
pub enum SaveMessageTag { #[serde(rename = "save_message")] Tag }

#[derive(Serialize, Debug)]
pub struct SaveMessage {
	pub message: SaveMessageTag,
	#[serde(with = "super::string")]
	pub id: NonZeroU64,
	#[serde(with = "super::string")]
	pub channel_id: NonZeroU64,
	#[serde(with = "super::string")]
	pub author_id: NonZeroU64,
	pub content: String,
	pub attachment_urls: Vec<String>
}
impl ClientMessage for SaveMessage {}

#[derive(Serialize, Debug)]
pub enum GetMessageTag { #[serde(rename = "get_message")] Tag }

#[derive(Serialize, Debug)]
pub struct GetMessage {
	pub message: GetMessageTag,
	#[serde(with = "super::string")]
	pub id: NonZeroU64
}
impl ClientMessage for GetMessage {}
