//! structs and things for messages sent by the client to the server

// https://github.com/serde-rs/serde/issues/760
// workaround for now: https://github.com/serde-rs/serde/issues/760#issuecomment-499570311

use twilight_bot_utils::prelude::*;

use serde::Serialize;

pub trait ClientMessage: Serialize {}

#[derive(Serialize, Debug)]
pub enum SaveMessageTagEnum { #[serde(rename = "save_message")] Tag }
pub use SaveMessageTagEnum::Tag as SaveMessageTag;
#[derive(Serialize, Debug)]
pub struct SaveMessage {
	pub message: SaveMessageTagEnum,
	pub id: Id::<MessageMarker>,
	pub channel_id: Id::<ChannelMarker>,
	pub author_id: Id::<UserMarker>,
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
	pub id: Id::<MessageMarker>
}
impl ClientMessage for GetMessage {}

#[derive(Serialize, Debug)]
pub enum SaveUserMessageTagEnum { #[serde(rename = "save_user_message")] Tag }
pub use SaveUserMessageTagEnum::Tag as SaveUserMessageTag;
#[derive(Serialize, Debug)]
pub struct SaveUserMessage {
	pub message: SaveUserMessageTagEnum,
	pub id: Id::<UserMarker>,
	pub name: String,
	#[serde(with = "super::discriminator")]
	pub discriminator: u16
}
impl ClientMessage for SaveUserMessage {}

#[derive(Serialize, Debug)]
pub enum GetUserMessageTagEnum { #[serde(rename = "get_user_message")] Tag }
pub use GetUserMessageTagEnum::Tag as GetUserMessageTag;
#[derive(Serialize, Debug)]
pub struct GetUserMessage {
	pub message: GetUserMessageTagEnum,
	pub id: Id::<UserMarker>
}
impl ClientMessage for GetUserMessage {}
