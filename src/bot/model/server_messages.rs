//! structs and things for messages sent by the server to the client

// https://github.com/serde-rs/serde/issues/760
// workaround for now: https://github.com/serde-rs/serde/issues/760#issuecomment-499570311

use twilight_bot_utils::prelude::*;
use serde::Deserialize;
pub trait ServerMessage: serde::de::DeserializeOwned {}


#[derive(Deserialize, Debug)]
pub struct Ready {
	pub message: ReadyTagEnum
}
#[derive(Deserialize, Debug)]
pub enum ReadyTagEnum { #[serde(rename = "ready")] Tag }
pub use ReadyTagEnum::Tag as ReadyTag;
impl ServerMessage for Ready {}


#[derive(Deserialize, Debug)]
pub struct Ok {
	pub message: OkTagEnum
}
#[derive(Deserialize, Debug)]
pub enum OkTagEnum { #[serde(rename = "ok")] Tag }
pub use OkTagEnum::Tag as OkTag;
impl ServerMessage for Ok {}


#[derive(Deserialize, Debug)]
pub struct Error {
	pub message: ErrorTagEnum,
	pub error: String
}
#[derive(Deserialize, Debug)]
pub enum ErrorTagEnum { #[serde(rename = "error")] Tag }
pub use ErrorTagEnum::Tag as ErrorTag;
impl ServerMessage for Error {}


#[derive(Deserialize, Debug)]
pub struct Message {
	pub message: MessageTagEnum,
	pub id: Id::<MessageMarker>,
	pub channel_id: Id::<ChannelMarker>,
	pub author_id: Id::<UserMarker>,
	pub content: String,
	pub attachment_urls: Vec<String>
}
#[derive(Deserialize, Debug)]
pub enum MessageTagEnum { #[serde(rename = "message")] Tag }
pub use MessageTagEnum::Tag as MessageTag;
impl ServerMessage for Message {}


#[derive(Deserialize, Debug)]
pub struct NoMessage {
	pub message: NoMessageTagEnum
}
#[derive(Deserialize, Debug)]
pub enum NoMessageTagEnum { #[serde(rename = "no_message")] Tag }
pub use NoMessageTagEnum::Tag as NoMessageTag;
impl ServerMessage for NoMessage {}


#[derive(Deserialize, Debug)]
pub struct User {
	pub message: UserTagEnum,
	pub id: Id::<UserMarker>,
	pub name: String,
	#[serde(with = "super::discriminator")]
	pub discriminator: u16,
	pub avatar_url: String
}
#[derive(Deserialize, Debug)]
pub enum UserTagEnum { #[serde(rename = "user")] Tag }
pub use UserTagEnum::Tag as UserTag;
impl ServerMessage for User {}


#[derive(Deserialize, Debug)]
pub struct NoUser {
	pub message: NoUserTagEnum
}
#[derive(Deserialize, Debug)]
pub enum NoUserTagEnum { #[serde(rename = "no_user")] Tag }
pub use NoUserTagEnum::Tag as NoUserTag;
impl ServerMessage for NoUser {}
