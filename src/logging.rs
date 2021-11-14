use twilight_bot_utils::prelude::*;

pub struct Logging();

#[async_trait]
impl Module for Logging {
	async fn handle_event(&self, event: Event) -> MainResult {
		match event.event {
			MessageCreate(msg) => {
				println!("message: {}", msg.content);
			}

			_ => {}
		}

		Ok(())
	}
}
