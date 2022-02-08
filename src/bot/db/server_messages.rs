//! structs and things for messages sent by the server to the client

// https://github.com/serde-rs/serde/issues/760
// workaround for now: https://github.com/serde-rs/serde/issues/760#issuecomment-499570311

use twilight_bot_utils::prelude::*;

use serde::Deserialize;
use serde::de::DeserializeOwned;

pub trait ServerMessage: DeserializeOwned {}


#[derive(Deserialize, Debug)]
pub struct Ready {
	pub message: ReadyTag
}
#[derive(Deserialize, Debug)]
pub enum ReadyTag { #[serde(rename = "ready")] Tag }
impl ServerMessage for Ready {}


#[derive(Deserialize, Debug)]
pub struct Ok {
	pub message: OkTag
}
#[derive(Deserialize, Debug)]
pub enum OkTag { #[serde(rename = "ok")] Tag }
impl ServerMessage for Ok {}


#[derive(Deserialize, Debug)]
pub struct Error {
	pub error: String
}
#[derive(Deserialize, Debug)]
pub enum ErrorTag { #[serde(rename = "error")] Tag }
impl ServerMessage for Error {}


#[derive(Deserialize, Debug)]
pub struct Message {
	pub message: MessageTag,
	pub id: Id::<MessageMarker>,
	pub channel_id: Id::<ChannelMarker>,
	pub author_id: Id::<UserMarker>,
	pub content: String,
	pub attachment_urls: Vec<String>
}
#[derive(Deserialize, Debug)]
pub enum MessageTag { #[serde(rename = "message")] Tag }
impl ServerMessage for Message {}


#[derive(Deserialize, Debug)]
pub struct NoMessage {
	pub message: NoMessageTag
}
#[derive(Deserialize, Debug)]
pub enum NoMessageTag { #[serde(rename = "no_message")] Tag }
impl ServerMessage for NoMessage {}


#[derive(Deserialize, Debug)]
pub struct User {
	pub message: UserTag,
	pub id: Id::<UserMarker>,
	pub name: String,
	#[serde(with = "super::discriminator")]
	pub discriminator: u16,
	pub avatar_url: String
}
#[derive(Deserialize, Debug)]
pub enum UserTag { #[serde(rename = "user")] Tag }
impl ServerMessage for User {}


#[derive(Deserialize, Debug)]
pub struct NoUser {
	pub message: NoUserTag
}
#[derive(Deserialize, Debug)]
pub enum NoUserTag { #[serde(rename = "no_user")] Tag }
impl ServerMessage for NoUser {}
