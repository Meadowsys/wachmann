// todo remove this when wachmann is more finished
#![allow(unused)]

mod env;

use std::error::Error;
use std::time::Duration;

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

	Ok(())
}

async fn async_main() -> Result<(), Box<dyn Error + Send + Sync>> {
	println!("runtime works lol");

	let env = env::Env::get_env()?;

	Ok(())
}
