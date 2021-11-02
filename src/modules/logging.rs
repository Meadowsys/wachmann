use crate::module::Event;
use futures::FutureExt;
use twilight_model::id::ChannelId;
use twilight_embed_builder::EmbedBuilder;

pub struct Logging {
	log_channel: ChannelId
}

impl Logging {
	pub fn run(&self, e: Event) {
		use twilight_gateway::Event::*;
		match e.event {
			_ => {}
		}
	}

	fn log_to_channel(&self, e: &Event) {
		// let embed = EmbedBuilder::new()
	}
}
