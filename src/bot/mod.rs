mod db;
mod env;
mod logging;
mod model;
mod webhook;

use twilight_bot_utils::prelude::*;

pub fn main() -> MainResult {
	println!("starting up...");

	let rt = twilight_bot_utils::rt::make_tokio_runtime();
	rt.block_on(async_main())?;
	rt.shutdown_timeout(std::time::Duration::from_secs(60));

	println!("down!");

	Ok(())
}

async fn async_main() -> MainResult {
	let http = twilight_bot_utils::http::setup_http()?;
	let intents = Intents::all();
	let (cluster, events) = twilight_bot_utils::cluster::setup_cluster(&intents).await?;
	let current_user = twilight_bot_utils::http::get_current_user(&http).await?;

	let db = db::Database::connect("db_service.sock").await;
	let db = match db {
		Ok(db) => { db }
		Err(ref err) => {
			eprint!("failed to connect to db: {err:?}");
			std::process::exit(1);
		}
	};
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
		// debug-specific things
	}

	#[cfg(not(debug_assertions))] {
		// production-specific things
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
