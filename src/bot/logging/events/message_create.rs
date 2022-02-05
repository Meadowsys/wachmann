use twilight_bot_utils::prelude::*;

use crate::bot::db::Database;
use crate::bot::db::client_messages;
use twilight_model::gateway::payload::incoming::MessageCreate;

pub async fn handle(msg: Box<MessageCreate>, db: &Arc<Database>) -> MainResult {
	let id = msg.id;
	let channel_id = msg.channel_id;
	let author_id = msg.author.id;
	let attachment_urls = msg.attachments.iter().map(|a| a.url.clone()).collect::<Vec<_>>();

	let content = msg.content
		.replace('*', "\\*")
		// .replace('#', "\\#") // do we really need this? it ruins the channel mentions
		.replace('/', "\\/")
		.replace('(', "\\(")
		.replace(')', "\\)")
		.replace('[', "\\[")
		.replace(']', "\\]")
		.replace('_', "\\_");

	db.save_message(&client_messages::SaveMessage {
		id, channel_id, author_id, content, attachment_urls,
		message: client_messages::SaveMessageTag
	}).await?;

	Ok(())
}
