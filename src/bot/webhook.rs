use twilight_bot_utils::prelude::*;

use twilight_http::client::Client;
use twilight_http::request::channel::webhook::ExecuteWebhook;
use twilight_model::user::CurrentUser;

pub struct Options<'a> {
	pub name: &'a str,
	pub avatar_url: &'a str,
	pub http: &'a Client, // TODO CHANGE THIS
	pub webhook_token: &'a str,
	pub webhook_id: Id<WebhookMarker>,
}

/// returns (name, avatar_url)
pub fn create_profile(current_user: &CurrentUser) -> (String, String) {
	let user_id = current_user.id;
	let avatar_hash = current_user.avatar;
	let (animated, hash) = avatar_hash
		.map(|h| (h.is_animated(), h.to_string()))
		.unwrap_or_else(|| (false, "https://cdn.discordapp.com/embed/avatars/1.png".into()));
	let ext = if animated { "gif" } else { "png" };

	(current_user.name.clone(), format!("https://cdn.discordapp.com/avatars/{user_id}/{hash}.{ext}"))
}

pub fn create_execute_webhook(Options { name, avatar_url, http, webhook_id, webhook_token }: Options<'_>) -> ExecuteWebhook {
	http.execute_webhook(webhook_id, webhook_token)
		.username(&name)
		.avatar_url(&avatar_url)
}
