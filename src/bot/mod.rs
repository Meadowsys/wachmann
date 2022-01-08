// todo remove this when wachmann is more finished
#![allow(unused)]

mod db;
mod env;
mod logging;

use twilight_bot_utils::prelude::*;

use db::server_messages::ServerMessage;
use std::io::Read;
use std::io::Write;
use std::os::unix::net::UnixStream;
use std::time::Duration;

fn tmp_read_next_line(stream: &mut UnixStream) -> MainResult<String> {
	let mut read_bytes = vec![];
	let mut next_byte = [0u8; 1];

	loop {
		let read_bytes_num = stream.read(&mut next_byte)?;
		if read_bytes_num == 0 || next_byte[0] == b'\n' {
			return Ok(String::from_utf8(read_bytes)?)
		}

		read_bytes.push(next_byte[0]);
	}
}

pub fn main() -> MainResult {
	// let mut socket = UnixStream::connect("db_service.sock")
	// 	.map_err(|_| "could not connect to db_service.sock, wachmann will not start up. if the database connector is running, then restart it and start wachmann again")?;
	// let read_str = tmp_read_next_line(&mut socket)?;
	// println!("{}", read_str);
	// let deserialised = serde_json::from_str::<ServerMessage>(&read_str)?;
	// println!("{:?}", deserialised);

	// let message = client_messages::ClientMessage::PutTestData {
	// 	data: "äääääääääääääääääää".into()
	// };

	// socket.write(serde_json::to_string(&message)?.as_bytes())?;
	// socket.write(b"\n")?;
	// println!("reading next message");
	// let read_str = tmp_read_next_line(&mut socket)?;
	// println!("{}", read_str);
	// let deserialised = serde_json::from_str::<ServerMessage>(&read_str)?;
	// println!("{:?}", deserialised);
	// drop(socket);

	let rt = twilight_bot_utils::rt::make_tokio_runtime();

	rt.block_on(async_main())?;

	Ok(())
}

async fn async_main() -> MainResult {
	println!("starting up...");

	let env = Env::get_env();
	let http = twilight_bot_utils::http::setup_http(&env)?;
	let intents = Intents::all();
	let (cluster, events) = twilight_bot_utils::cluster::setup_cluster(&env, &intents).await?;
	let current_user = twilight_bot_utils::http::get_current_user(&http).await?;

	let db = db::Database::connect("db_service.sock").await?;
	let db = Arc::new(db);

	let mut modules = ModuleHandler::with_capacity(10);

	modules.add_module(logging::new(
		env_var("LOGGING_WEBHOOK")
			.or("WEBHOOK_URL")
			.or("WACHMANN_WEBHOOK_URL")
			.get(),
		db
	));

	#[cfg(debug_assertions)] {
		// debug-specific modules
	}

	#[cfg(not(debug_assertions))] {
		// production-specific modules
	}

	let modules = modules
		.init_modules(&http, &current_user)
		.await?
		.into_modules();

	cluster.up().await;
	println!("up!");

	let cluster_down = cluster.clone();
	spawn(watch_for_shutdown_signals(move |sig| {
		println!("received {}, shutting down...", sig);
		cluster_down.down();
	}));

	process_events(events, http, modules).await;

	println!("down!");
	Ok(())
}
