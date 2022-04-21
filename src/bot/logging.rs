use twilight_bot_utils::prelude::*;
use super::db::Database;
use twilight_http::Client as HttpClient;
use twilight_model::user::CurrentUser;
use twilight_util::link::webhook::parse;

mod events;

pub struct Logging {
	webhook_id: Id<WebhookMarker>,
	webhook_token: String,
	current_user: CurrentUser,
	channel_id: Id::<ChannelMarker>,
	db: Arc<Database>
}

#[inline]
pub fn new(webhook_url: String, db: Arc<Database>) -> Logging {
	Logging::new(webhook_url, db)
}

#[allow(clippy::new_without_default)]
impl Logging {
	pub fn new(webhook_url: String, db: Arc<Database>) -> Self {
		let (webhook_id, webhook_token) = twilight_util::link::webhook::parse(&webhook_url).unwrap();

		Self {
			webhook_id,
			webhook_token: webhook_token.unwrap().into(),
			current_user: CurrentUser {
				accent_color: None,
				avatar: None,
				banner: None,
				bot: false,
				discriminator: 0,
				email: None,
				id: Id::<UserMarker>::new(1),
				mfa_enabled: false,
				name: "not initialised".to_owned(),
				verified: None,
				premium_type: None,
				public_flags: None,
				flags: None,
				locale: None
			},
			channel_id: Id::<ChannelMarker>::new(1),
			db
		}
	}
}

#[async_trait]
impl Module for Logging {
	async fn init(&mut self, stuff: &InitStuff) -> InitResult {
		self.current_user = stuff.current_user.clone();

		self.channel_id = stuff.http.webhook(self.webhook_id)
			.exec().await?
			.model().await?
			.channel_id;

		Ok(())
	}

	async fn handle_event(&self, event: Event) -> HandleResult {
		// i add all events manually so that when something is added/removed, i will
		// notice for sure because the build breaks

		match event.event {
			MessageCreate(ref msg) => {
				if self.channel_id == msg.channel_id { return Ok(()) }
				if self.current_user.id == msg.author.id { return Ok(()) }
				events::message_create::handle(msg, &self.db).await?;
			}

			MessageUpdate(ref msg) => {
				if self.channel_id == msg.channel_id { return Ok(()) }
				if let Some(ref author) = msg.author {
					if self.current_user.id == author.id { return Ok(()) }
				} else { return Ok(()) }
				events::message_update::handle(msg, &event, &self).await?;
			}

			// unused events
			BanAdd(_) => {}
			BanRemove(_) => {}
			ChannelCreate(_) => {}
			ChannelDelete(_) => {}
			ChannelPinsUpdate(_) => {}
			ChannelUpdate(_) => {}
			GatewayHeartbeat(_) => {}
			GatewayHeartbeatAck => {}
			GatewayHello(_) => {}
			GatewayInvalidateSession(_) => {}
			GatewayReconnect => {}
			GuildCreate(_) => {}
			GuildDelete(_) => {}
			GuildEmojisUpdate(_) => {}
			GuildIntegrationsUpdate(_) => {}
			GuildUpdate(_) => {}
			IntegrationCreate(_) => {}
			IntegrationDelete(_) => {}
			IntegrationUpdate(_) => {}
			InteractionCreate(_) => {}
			InviteCreate(_) => {}
			InviteDelete(_) => {}
			MemberAdd(_) => {}
			MemberRemove(_) => {}
			MemberUpdate(_) => {}
			MemberChunk(_) => {}
			MessageDelete(_) => {}
			MessageDeleteBulk(_) => {}
			PresenceUpdate(_) => {}
			ReactionAdd(_) => {}
			ReactionRemove(_) => {}
			ReactionRemoveAll(_) => {}
			ReactionRemoveEmoji(_) => {}
			Ready(_) => {}
			Resumed => {}
			RoleCreate(_) => {}
			RoleDelete(_) => {}
			RoleUpdate(_) => {}
			ShardConnected(_) => {}
			ShardConnecting(_) => {}
			ShardDisconnected(_) => {}
			ShardIdentifying(_) => {}
			ShardReconnecting(_) => {}
			ShardPayload(_) => {}
			ShardResuming(_) => {}
			StageInstanceCreate(_) => {}
			StageInstanceDelete(_) => {}
			StageInstanceUpdate(_) => {}
			ThreadCreate(_) => {}
			ThreadDelete(_) => {}
			ThreadListSync(_) => {}
			ThreadMemberUpdate(_) => {}
			ThreadMembersUpdate(_) => {}
			ThreadUpdate(_) => {}
			TypingStart(_) => {}
			UnavailableGuild(_) => {}
			UserUpdate(_) => {}
			VoiceServerUpdate(_) => {}
			VoiceStateUpdate(_) => {}
			WebhooksUpdate(_) => {}

			GiftCodeUpdate => { /* undocumented */ }
			PresencesReplace => { /* for bots, is always empty and useless */ }
		}

		Ok(())
	}
}
