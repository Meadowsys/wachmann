// todo remove this when wachmann is more finished
#![allow(unused)]

mod env;
mod logging;

use twilight_bot_utils::prelude::*;

use std::time::Duration;
use twilight_bot_utils::modules::ModuleHandler;
use twilight_bot_utils::run::watch_for_shutdown_signals;
use twilight_bot_utils::run::process_events;

fn main() -> MainResult {
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

	#[cfg(debug_assertions)] {
		modules.add_module(logging::Logging());
	}

	#[cfg(not(debug_assertions))] {
		// something
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
