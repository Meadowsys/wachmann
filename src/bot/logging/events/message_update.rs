use twilight_bot_utils::prelude::*;

use crate::bot::db::Database;
use crate::bot::db::client_messages;
use twilight_model::gateway::payload::incoming::MessageUpdate;

pub async fn handle(new_msg: &Box<MessageUpdate>, db: &Arc<Database>, event: &Event) -> MainResult {
	println!("message update");
	// println!("{:?}", new_msg.content);
	let old_msg = db.get_message(&client_messages::GetMessage {
		message: client_messages::GetMessageTag,
		id: new_msg.id
	}).await?;
	println!("{:?}", old_msg);
	println!("done");
	Ok(())
}
