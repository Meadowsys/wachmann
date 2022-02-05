use twilight_bot_utils::prelude::*;

use crate::bot::db::Database;
use crate::bot::db::client_messages;
use twilight_model::gateway::payload::incoming::MessageUpdate;

pub async fn handle(new_msg: Box<MessageUpdate>, db: &Arc<Database>) -> MainResult {
	println!("message update");
	// println!("{:?}", new_msg.content);
	let old_msg = db.get_message(&client_messages::GetMessage {
		id: new_msg.id,
		message: client_messages::GetMessageTag
	}).await?;
	println!("{:?}", old_msg);
	println!("done");
	Ok(())
}
