//! structs and things for messages sent by the server to the client

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(tag = "message")]
#[serde(rename_all = "snake_case")]
pub enum ServerMessage {
	Ready {},
	TestData {
		id: String,
		data: String
	}
}
