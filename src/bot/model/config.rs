use twilight_bot_utils::prelude::*;
use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
	pub main_log_channel: Option<Id<ChannelMarker>>,
	pub main_webhook_id: Option<Id<WebhookMarker>>,
	pub main_webhook_token: Option<String>
}
