// todo remove this when wachmann is more finished
#![allow(unused)]

mod env;
mod module;
mod modules;

use std::error::Error;
use std::time::Duration;
use twilight_model::channel::message::allowed_mentions::AllowedMentionsBuilder;
use twilight_http::client::ClientBuilder as HttpClientBuilder;
use twilight_gateway::{
	cluster::{ Cluster, ShardScheme::Auto },
	Intents
};
use futures::stream::StreamExt;
use module::Event;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
	let rt = tokio::runtime::Builder::new_multi_thread()
		.enable_all()
		.worker_threads(2)
		.max_blocking_threads(32)
		.thread_keep_alive(Duration::from_secs(60))
		.build()
		.unwrap();

	rt.block_on(async_main())?;
	rt.shutdown_timeout(Duration::from_secs(60));

	println!("down!");

	Ok(())
}

async fn async_main() -> Result<(), Box<dyn Error + Send + Sync>> {
	println!("starting up...");

	let env = env::Env::get_env()?;

	let allowed_mentions = AllowedMentionsBuilder::new()
		.users()
		.build();

	let http = HttpClientBuilder::new()
		.default_allowed_mentions(allowed_mentions)
		.build();

	let intents = Intents::all();

	let (cluster, mut events) = Cluster::builder(env.token(), intents)
		.shard_scheme(Auto)
		.build().await?;

	cluster.up().await;
	println!("up!");

	let cluster_down = cluster.clone();
	tokio::spawn(async move {
		use tokio::signal::unix::{ signal, SignalKind as SK };
		let mut sigint = signal(SK::interrupt()).unwrap();
		let mut sigterm = signal(SK::terminate()).unwrap();

		tokio::select! {
			// without biased, tokio::select! will choose random branches to poll,
			// which incurs a small cpu cost for the random number generator
			// biased polling is fine here
			biased;

			_ = sigint.recv() => {
				println!("received SIGINT, shutting down...");
			}
			_ = sigterm.recv() => {
				println!("received SIGTERM, shutting down...");
			}
		}

		cluster_down.down();
	});

	while let Some((shard_id, event)) = events.next().await {
		use module::s;
		let e = Event { shard_id, event, http: http.clone() };

		// s(&e, logging::logging);
	}

	Ok(())
}
