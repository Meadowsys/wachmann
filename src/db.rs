use twilight_bot_utils::prelude::*;

pub mod client_messages;
pub mod server_messages;

use client_messages::ClientMessage;
use server_messages::ServerMessage;
use twilight_bot_utils::prelude::tokio::io::AsyncWriteExt;
use std::error::Error;
use std::fmt;
use std::sync::Arc;
use tokio::net::UnixStream;
use tokio::io::AsyncReadExt;
use tokio::sync::Mutex;
use std::str;

#[derive(Debug)]
pub enum DatabaseConnectionError {
	UnexpectedEndOfStream,
	UnexpectedConnectionError
}

impl fmt::Display for DatabaseConnectionError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::UnexpectedEndOfStream => {
				write!(f, "unexpected end of stream")
			}
			Self::UnexpectedConnectionError => {
				write!(f, "unexpected error while connecting to database")
			}
		}
	}
}

impl Error for DatabaseConnectionError {}

struct DatabaseConnection {
	socket: UnixStream
}

impl DatabaseConnection {
	pub async fn connect(path: &str) -> MainResult<Self> {
		let socket = UnixStream::connect(path).await?;
		let mut connection = Self { socket };

		match connection.read_next_message().await {
			Ok(ServerMessage::Ready { .. }) => { Ok(connection) }
			Ok(_) | Err(_) => { Err(Box::from(DatabaseConnectionError::UnexpectedConnectionError)) }
		}
	}

	pub async fn read_next_message(&mut self) -> MainResult<ServerMessage> {
		let line = self.read_next_line().await?;
		Ok(serde_json::from_str(&line)?)
	}

	async fn read_next_line(&mut self) -> MainResult<String> {
		let mut next_byte = [0u8; 1];
		let mut read_bytes = vec![];

		loop {
			let read_bytes_num = self.socket.read(&mut next_byte).await?;

			if read_bytes_num == 0 { return Err(Box::new(DatabaseConnectionError::UnexpectedEndOfStream)) }
			let eee: &str = "dsa";
			if next_byte[0] == b'\n' { return Ok(String::from_utf8(read_bytes)?) }

			read_bytes.push(next_byte[0]);
		}
	}

	async fn send_message(&mut self, message: &ClientMessage) -> MainResult {
		let stringified = serde_json::to_string(message)?;
		self.socket.write_all(stringified.as_bytes()).await?;
		Ok(())
	}
}

pub struct Database {
	path: String,
	connections: Arc<Mutex<Vec<DatabaseConnection>>>
}

impl Database {
	pub async fn connect(path: &str) -> MainResult<Self> {
		// connect our first socket to test the connection
		let db_connection = DatabaseConnection::connect(path).await?;
		Ok(Database {
			connections: Arc::new(Mutex::new(vec![db_connection])),
			path: path.into()
		})
	}

	#[inline]
	async fn _process_query(&mut self, query: &ClientMessage, connection: &mut DatabaseConnection) -> MainResult<ServerMessage> {
		connection.send_message(query).await?;
		let response = connection.read_next_message().await?;
		Ok(response)
	}

	#[inline]
	#[async_recursion::async_recursion]
	async fn process_query(&mut self, query: &ClientMessage) -> MainResult<ServerMessage> {
		let mut connections = self.connections.lock().await;
		let connection = connections.pop();
		drop(connections);
			// .unwrap_or_else(|_| DatabaseConnection::connect(&self.path).await?);

		let mut connection = if let Some(c) = connection { c }
		else { DatabaseConnection::connect(&self.path).await? };

		let processed = self._process_query(query, &mut connection).await;

		if let Ok(res) = processed {
			let mut connections = self.connections.lock().await;
			connections.push(connection);
			drop(connections);
			Ok(res)
		} else {
			self.process_query(query).await
		}
	}
}
