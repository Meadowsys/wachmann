// todo remove this when wachmann is more finished
#![allow(unused)]

mod env;
mod logging;

use twilight_bot_utils::prelude::*;

use std::os::unix::net::UnixStream;
use std::time::Duration;

fn main() -> MainResult {
	let mut socket = UnixStream::connect("db_service.sock").unwrap();
	drop(socket);

	let rt = twilight_bot_utils::rt::make_tokio_runtime();

	rt.block_on(async_main())?;
	rt.shutdown_timeout(Duration::from_secs(60));

	println!("down!");

	Ok(())
}

async fn async_main() -> MainResult {
	println!("starting up...");

	let env = Env::get_env();
	let http = twilight_bot_utils::http::setup_http(&env)?;
	let intents = Intents::all();
	let (cluster, events) = twilight_bot_utils::cluster::setup_cluster(&env, &intents).await?;
	let current_user = twilight_bot_utils::http::get_current_user(&http).await?;

	let mut modules = ModuleHandler::with_capacity(10);

	modules.add_module(logging::new(
		env_var("LOGGING_WEBHOOK")
			.or("WEBHOOK_URL")
			.or("WACHMANN_WEBHOOK_URL")
			.get()
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

	Ok(())
}
