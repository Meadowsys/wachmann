use twilight_bot_utils::prelude::*;

mod db;
mod modules;

use std::time::Duration;

pub fn main() -> MainResult {
	// first thing to do, print starting up message
	println!("starting up...");

	let rt = make_tokio_runtime();
	rt.block_on(async_main())?;
	rt.shutdown_timeout(Duration::from_secs(60));

	// this needs here instead of in async_main because of rt.shutdown_timeout
	// might wait for something
	println!("down!");

	Ok(())
}

async fn async_main() -> MainResult {
	let http = setup_http()?;
	let intents = Intents::all();
	let (cluster, events) = setup_cluster(&intents).await?;
	let current_user = get_current_user(&http).await?;

	// todo connect db
	let db = db::Database::spawn().await?;

	// todo create modules
	let mut modules = ModuleHandler::with_capacity(10);
	modules.add_module(modules::ready::ReadyListener);
	let modules = modules
		.init_modules(&http, &current_user).await?
		.into_modules();

	cluster.up().await;
	// print up once cluster is up
	println!("up!");

	let cluster_down = cluster.clone();
	spawn(watch_for_shutdown_signals(move |sig| {
		println!("received {sig}, shutting down...");
		cluster_down.down();
	}));

	process_events(events, http, modules).await;

	Ok(())
}
