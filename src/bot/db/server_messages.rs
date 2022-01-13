//! structs and things for messages sent by the server to the client

// https://github.com/serde-rs/serde/issues/760
// workaround for now: https://github.com/serde-rs/serde/issues/760#issuecomment-499570311

use twilight_bot_utils::prelude::*;

use serde::Deserialize;
use serde::de::DeserializeOwned;
use twilight_model::id::ChannelId;
use twilight_model::id::MessageId;
use twilight_model::id::UserId;

pub trait ServerMessage: DeserializeOwned {}

#[derive(Deserialize, Debug)]
pub enum ReadyTag { #[serde(rename = "ready")] Tag }
#[derive(Deserialize, Debug)]
pub struct Ready {
	pub message: ReadyTag
}
impl ServerMessage for Ready {}

#[derive(Deserialize, Debug)]
pub enum OkTag { #[serde(rename = "ok")] Tag }
#[derive(Deserialize, Debug)]
pub struct Ok {
	pub message: OkTag
}
impl ServerMessage for Ok {}

#[derive(Deserialize, Debug)]
pub enum ErrorTag { #[serde(rename = "error")] Tag }
#[derive(Deserialize, Debug)]
pub struct Error {
	pub error: String
}
impl ServerMessage for Error {}

#[derive(Deserialize, Debug)]
pub enum MessageTag { #[serde(rename = "message")] Tag }
#[derive(Deserialize, Debug)]
pub struct Message {
	pub message: MessageTag,
	pub id: MessageId,
	pub channel_id: ChannelId,
	pub author_id: UserId,
	pub content: String,
	pub attachment_urls: Vec<String>
}
impl ServerMessage for Message {}

#[derive(Deserialize, Debug)]
pub enum NoMessageTag { #[serde(rename = "no_message")] Tag }
#[derive(Deserialize, Debug)]
pub struct NoMessage {
	pub message: NoMessageTag
}
impl ServerMessage for NoMessage {}
