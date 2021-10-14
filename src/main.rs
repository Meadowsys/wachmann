// todo remove this when wachmann is more finished
#![allow(unused)]

use std::error::Error;
use std::time::Duration;

type MainResult = Result<(), Box<dyn Error + Send + Sync>>;

fn main() -> MainResult {
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

async fn async_main() -> MainResult {
	println!("runtime works lol");

	Ok(())
}
