use twilight_bot_utils::prelude::*;

use crate::bot::db::Database;
use crate::bot::db::client_messages::ClientMessage;
use twilight_model::gateway::payload::incoming::MessageCreate;

pub async fn handle(msg: Box<MessageCreate>, db: &Arc<Database>) -> MainResult {
	let id = msg.id.0;
	let channel_id = msg.channel_id.0;
	let author_id = msg.author.id.0;
	let attachment_urls = msg.attachments.iter().map(|a| a.url.clone()).collect::<Vec<_>>();

	let content = msg.content
		.replace('*', "\\*")
		.replace('#', "\\#")
		.replace('/', "\\/")
		.replace('(', "\\(")
		.replace(')', "\\)")
		.replace('[', "\\[")
		.replace(']', "\\]")
		.replace('_', "\\_");

	db.save_message(&ClientMessage::SaveMessage { id, channel_id, author_id, content, attachment_urls }).await?;

	Ok(())
}
