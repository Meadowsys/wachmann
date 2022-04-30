use twilight_bot_utils::prelude::*;

mod error;
use std::include_str;
use error::*;
use nix::sys::signal::kill;
use nix::sys::signal::SIGINT;
use nix::unistd::Pid;
use std::path;
use std::process::Stdio;
use tokio::fs;
use tokio::io::AsyncReadExt;
use tokio::process::ChildStdout;
use tokio::process;

pub struct Database {
	db_script_filename: String,
	db_process: process::Child
}

impl Database {
	pub async fn spawn() -> MainResult<Self> {
		#[cfg(debug_assertions)]
		let db_script = include_str!("../../target/debug/db.mjs");

		#[cfg(not(debug_assertions))]
		let db_script = include_str!("../../target/release/db.mjs");

		let db_script_filename = {
			let i = 0;
			loop {
				// "dbscript-{}.mjs"
				// 9 + length of i + 4
				// lets just call "length of i" to be like, 2
				// (don't really think i'm going to be running 100 wachmann instances with the same cwd?)
				let mut filename = String::with_capacity(9 + 2 + 4);
				filename.push_str("dbscript-");
				filename.push_str(&i.to_string());
				filename.push_str(".mjs");

				if !path::Path::new(&filename).exists() { break filename }
			}
		};

		fs::write(&db_script_filename, db_script).await?;

		let mut db_process = process::Command::new("node")
			.arg(&db_script_filename)
			.stdout(Stdio::piped())
			.spawn()?;

		let mut stdout = db_process.stdout.take().ok_or(Box::new(DatabaseConnectionError::ChildProcessNoStdout))?;

		async fn read_line_of_stdout(stdout: &mut ChildStdout) -> MainResult<String> {
			let mut next_byte = [0u8; 1];
			let mut read_bytes = Vec::with_capacity(512);
			loop {
				let num_of_read_bytes = stdout.read(&mut next_byte).await?;
				if num_of_read_bytes == 0 { break Err(Box::new(DatabaseConnectionError::UnexpectedStdoutEnd)) }
				if next_byte[0] == b'\n' { break Ok(String::from_utf8(read_bytes)?) }
				read_bytes.push(next_byte[0]);
			}
		}

		let sock_path = read_line_of_stdout(&mut stdout).await?;
		let secret = read_line_of_stdout(&mut stdout).await?;

		Ok(Database { db_script_filename, db_process })
	}
}

impl Drop for Database {
	#[allow(unused_must_use)]
	fn drop(&mut self) {
		std::fs::remove_file(&self.db_script_filename);
		let process_id = self.db_process.id().unwrap();
		kill(Pid::from_raw(process_id as i32), SIGINT);
		futures::executor::block_on(self.db_process.wait());
	}
}
