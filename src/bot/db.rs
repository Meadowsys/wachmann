use twilight_bot_utils::prelude::*;

mod error;

use error::*;
use nix::sys::signal::kill;
use nix::sys::signal::SIGINT;
use nix::unistd::Pid;
use std::process::Stdio;
use tokio::io::AsyncReadExt;
use tokio::process;

pub struct Database {
	db_process: process::Child
}

impl Database {
	pub async fn spawn() -> MainResult<Self> {
		let mut db_script = "./db.mjs";
		if !std::path::Path::new(db_script).exists() {
			#[cfg(debug_assertions)]
			{ db_script = "./target/debug/db.mjs"; }

			#[cfg(not(debug_assertions))]
			{ db_script = "./target/release/db.mjs"; }

			if !std::path::Path::new(db_script).exists() { return Err(Box::new(DatabaseConnectionError::DbScriptNotFound)) }
		}


		let mut db_process = process::Command::new("node")
			.arg(db_script)
			.stdout(Stdio::piped())
			.spawn()?;

		let mut stdout = db_process.stdout.take().ok_or(Box::new(DatabaseConnectionError::ChildProcessNoStdout))?;
		let mut next_byte = [0u8; 1];
		let mut read_bytes = vec![];

		let sock_path: MainResult<String> = loop {
			let read_bytes_num = stdout.read(&mut next_byte).await?;

			if read_bytes_num == 0 { break Err(Box::new(DatabaseConnectionError::UnexpectedStdoutEnd)) }
			if next_byte[0] == b'\n' { break Ok(String::from_utf8(read_bytes)?) }

			read_bytes.push(next_byte[0]);
		};

		Ok(Database { db_process })
	}
}

impl Drop for Database {
	#[allow(unused_must_use)]
	fn drop(&mut self) {
		let process_id = self.db_process.id().unwrap();
		kill(Pid::from_raw(process_id as i32), SIGINT);
		futures::executor::block_on(self.db_process.wait());
	}
}
