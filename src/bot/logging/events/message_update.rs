use twilight_bot_utils::prelude::*;

use crate::bot::db::Database;
use crate::bot::db::client_messages::ClientMessage;
use twilight_model::gateway::payload::incoming::MessageUpdate;

pub async fn handle(new_msg: Box<MessageUpdate>, db: &Arc<Database>) -> MainResult {
	// println!("{:?}", new_msg.content);
	let old_msg = db.get_message(&ClientMessage::GetMessage {
		id: new_msg.id.0
	}).await?;
	println!("{:?}", old_msg);
	println!("done");
	Ok(())
}
