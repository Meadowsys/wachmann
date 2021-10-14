use crate::module::Event;

pub async fn logging(event: Event) {
	use twilight_gateway::Event::*;

	match event.event {
		MessageCreate(msg) => {
			println!("got msg {}", msg.content);
		}
		_ => {}
	}
}
