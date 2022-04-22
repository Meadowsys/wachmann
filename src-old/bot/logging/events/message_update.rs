use twilight_bot_utils::prelude::*;

use crate::bot::db::Database;
use crate::bot::db::client_messages;
use crate::bot::webhook::create_profile;
use crate::bot::webhook::create_execute_webhook;
use crate::bot::webhook::Options as WebhookOptions;
use super::super::Logging;
use twilight_embed_builder::EmbedAuthorBuilder;
use twilight_embed_builder::EmbedBuilder;
use twilight_embed_builder::EmbedFieldBuilder;
use twilight_embed_builder::EmbedFooterBuilder;
use twilight_embed_builder::ImageSource;
use twilight_model::gateway::payload::incoming::MessageUpdate;
use twilight_model::user::CurrentUser;

pub async fn handle(new_msg: &Box<MessageUpdate>, event: &Event, Logging { current_user, db, webhook_id, webhook_token, .. }: &Logging) -> MainResult {
	println!("message update");
	// println!("{:?}", new_msg.content);
	let old_msg = db.get_message(&client_messages::GetMessage {
		message: client_messages::GetMessageTag,
		id: new_msg.id
	}).await?;

	let (name, avatar_url) = create_profile(current_user);
	create_execute_webhook(WebhookOptions {
		name: &name,
		avatar_url: &avatar_url,
		http: &*event.http,
		webhook_id: *webhook_id,
		webhook_token
	})
		.content("test")?
		.exec().await?;

	println!("{:?}", old_msg);
	println!("done");
	Ok(())
}
