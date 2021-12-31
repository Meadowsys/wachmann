//! structs and things for messages sent by the client to the server

use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(tag = "message")]
#[serde(rename_all = "snake_case")]
pub enum ClientMessage {
	PutTestData {
		data: String
	},
	GetTestData {
		id: String
	}
}
