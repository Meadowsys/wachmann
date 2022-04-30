use twilight_bot_utils::prelude::*;

// world's most useless module because like the main program itself already does it
// temporary placeholder then, lol
pub struct ReadyListener;

#[async_trait]
impl Module for ReadyListener {
	async fn handle_event(&self, event: Event) -> HandleResult {
		if let Ready(_) = event.event {
			println!("connected!");
		}
		Ok(())
	}
}
