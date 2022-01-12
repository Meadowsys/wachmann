use twilight_bot_utils::prelude::*;

pub mod client_messages;
pub mod server_messages;

use client_messages::ClientMessage;
use server_messages::ServerMessage;
use twilight_bot_utils::prelude::tokio::io::AsyncWriteExt;
use std::error::Error;
use std::fmt;
use tokio::net::UnixStream;
use tokio::io::AsyncReadExt;
use tokio::sync::Mutex;
use std::str;

#[derive(Debug)]
pub enum DatabaseError {
	UnexpectedEndOfStream,
	UnexpectedConnectionError,
	// InvalidMessage,
	// UnexpectedMessage
}

impl fmt::Display for DatabaseError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::UnexpectedEndOfStream => {
				write!(f, "unexpected end of stream")
			}
			Self::UnexpectedConnectionError => {
				write!(f, "unexpected error while connecting to database")
			}
			// Self::InvalidMessage => {
			// 	write!(f, "invalid message type provided")
			// }
			// Self::UnexpectedMessage => {
			// 	write!(f, "unexpected message type received, unsure if successful or not")
			// }
		}
	}
}

impl Error for DatabaseError {}

struct DatabaseConnection {
	socket: UnixStream
}

impl DatabaseConnection {
	pub async fn connect(path: &str) -> MainResult<Self> {
		let socket = UnixStream::connect(path).await?;
		let mut connection = Self { socket };

		match connection.read_next_message::<server_messages::Ready>().await {
			Ok(_) => { Ok(connection) }
			Err(_) => { Err(Box::new(DatabaseError::UnexpectedConnectionError)) }
		}
	}

	pub async fn read_next_message<T: ServerMessage>(&mut self) -> MainResult<T> {
		let line = self.read_next_line().await?;
		Ok(serde_json::from_str(&line)?)
	}

	pub async fn read_next_line(&mut self) -> MainResult<String> {
		let mut next_byte = [0u8; 1];
		let mut read_bytes = vec![];

		loop {
			let read_bytes_num = self.socket.read(&mut next_byte).await?;

			if read_bytes_num == 0 { return Err(Box::new(DatabaseError::UnexpectedEndOfStream)) }
			if next_byte[0] == b'\n' { return Ok(String::from_utf8(read_bytes)?) }

			read_bytes.push(next_byte[0]);
		}
	}

	async fn send_message<T: ClientMessage>(&mut self, message: &T) -> MainResult {
		let stringified = serde_json::to_string(message)?;
		self.socket.write_all(stringified.as_bytes()).await?;
		self.socket.write_all(b"\n").await?;
		Ok(())
	}
}

pub struct Database {
	path: String,
	connections: Mutex<Vec<DatabaseConnection>>
}

impl Database {
	pub async fn connect(path: &str) -> MainResult<Self> {
		// connect our first socket to test the connection
		let db_connection = DatabaseConnection::connect(path).await?;

		let mut vec = Vec::with_capacity(3);
		vec.push(db_connection);

		Ok(Database {
			connections: Mutex::new(vec),
			path: path.into()
		})
	}

	#[inline]
	async fn get_connection(&self) -> MainResult<DatabaseConnection> {
		let mut connections = self.connections.lock().await;
		let connection = connections.pop();
		drop(connections);

		let connection = if let Some(con) = connection {
			con
		} else {
			DatabaseConnection::connect(&self.path).await?
		};

		Ok(connection)
	}

	#[inline]
	async fn return_connection(&self, connection: DatabaseConnection) {
		let mut connections = self.connections.lock().await;
		connections.push(connection);
	}

	async fn process_query_no_parse_once<T>(&self, query: &T) -> MainResult<String>
	where
		T: ClientMessage
	{
		let mut connection = self.get_connection().await?;

		connection.send_message(query).await?;
		let response = connection.read_next_line().await;

		if let Ok(_) = response {
			self.return_connection(connection).await;
		}
		// if it errored, i suppose it might be something with that connection?
		// so don't return it and let it drop/disconnect

		response
	}

	#[inline]
	async fn process_query_no_parse<T>(&self, query: &T)-> MainResult<String>
	where
		T: ClientMessage
	{
		let mut response = self.process_query_no_parse_once(query).await;

		for _ in 0..5 {
			if let Ok(_) = response { return response }
			response = self.process_query_no_parse_once(query).await;
		}

		response
	}

	async fn process_query_once<T, R>(&self, query: &T) -> MainResult<R>
	where
		T: ClientMessage,
		R: ServerMessage
	{
		let mut connection = self.get_connection().await?;

		connection.send_message(query).await?;
		let processed = connection.read_next_message().await;

		if let Ok(_) = processed {
			self.return_connection(connection).await;
		}
		// if it errored, i suppose it might be something with that connection?
		// so don't return it and let it drop/disconnect

		processed
	}

	#[inline]
	async fn process_query<T, R>(&self, query: &T) -> MainResult<R>
	where
		T: ClientMessage,
		R: ServerMessage
	{
		let mut result = self.process_query_once(query).await;

		for _ in 0..5 {
			if let Ok(response) = result { return Ok(response) }
			result = self.process_query_once(query).await;
		}

		result
	}

	#[inline]
	pub async fn save_message(&self, msg: &client_messages::SaveMessage)
		-> MainResult<server_messages::Ok>
	{
		self.process_query(msg).await
	}

	#[inline]
	pub async fn get_message(&self, msg: &client_messages::GetMessage)
		-> MainResult<Option<server_messages::Message>>
	{
		let str_res = self.process_query_no_parse(msg).await?;
		match serde_json::from_str::<server_messages::Message>(&str_res) {
			Ok(res) => { Ok(Some(res)) }
			Err(e) => {
				let no_msg_res = serde_json::from_str::<server_messages::NoMessage>(&str_res);
				match no_msg_res {
					Ok(_) => { Ok(None) }
					Err(_) => { Err(Box::new(e)) }
				}
			}
		}
	}
}

/// almost straight copy/paste from twilight_model/src/id.rs
pub(self) mod string {
	use serde::de::Deserializer;
	use serde::de::Error as DeError;
	use serde::de::Unexpected;
	use serde::de::Visitor;
	use serde::ser::Serializer;
	use std::fmt::Display;
	use std::fmt::Formatter;
	use std::fmt::Result as FmtResult;
	use std::marker::PhantomData;
	use std::num::NonZeroU64;

	struct IdVisitor<T: From<NonZeroU64>>(PhantomData<T>);

	impl<'de, T: From<NonZeroU64>> Visitor<'de> for IdVisitor<T> {
		type Value = T;

		fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
			f.write_str("string or integer snowflake")
		}

		fn visit_u64<E: DeError>(self, value: u64) -> Result<Self::Value, E> {
			NonZeroU64::new(value).map(T::from).ok_or_else(|| {
				let unexpected = Unexpected::Unsigned(value);

				DeError::invalid_value(unexpected, &"a non-zero unsigned integer")
			})
		}

		fn visit_str<E: DeError>(self, value: &str) -> Result<Self::Value, E> {
			value.parse().map(T::from).map_err(DeError::custom)
		}
	}

	pub fn serialize<T: Display, S: Serializer>(
		value: &T,
		serializer: S,
	) -> Result<S::Ok, S::Error> {
		serializer.collect_str(value)
	}

	pub fn deserialize<'de, T: From<NonZeroU64>, D: Deserializer<'de>>(
		deserializer: D,
	) -> Result<T, D::Error> {
		deserializer.deserialize_any(IdVisitor(PhantomData))
	}
}
