//! structs and things for messages sent by the client to the server

// https://github.com/serde-rs/serde/issues/760
// workaround for now: https://github.com/serde-rs/serde/issues/760#issuecomment-499570311

use twilight_bot_utils::prelude::*;

use serde::Serialize;
use twilight_model::id::ChannelId;
use twilight_model::id::MessageId;
use twilight_model::id::UserId;

pub trait ClientMessage: Serialize {}

#[derive(Serialize, Debug)]
pub enum SaveMessageTagEnum { #[serde(rename = "save_message")] Tag }
pub use SaveMessageTagEnum::Tag as SaveMessageTag;
#[derive(Serialize, Debug)]
pub struct SaveMessage {
	pub message: SaveMessageTagEnum,
	pub id: MessageId,
	pub channel_id: ChannelId,
	pub author_id: UserId,
	pub content: String,
	pub attachment_urls: Vec<String>
}
impl ClientMessage for SaveMessage {}

#[derive(Serialize, Debug)]
pub enum GetMessageTagEnum { #[serde(rename = "get_message")] Tag }
pub use GetMessageTagEnum::Tag as GetMessageTag;
#[derive(Serialize, Debug)]
pub struct GetMessage {
	pub message: GetMessageTagEnum,
	pub id: MessageId
}
impl ClientMessage for GetMessage {}
