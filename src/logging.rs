use twilight_bot_utils::prelude::*;
use twilight_http::Client as HttpClient;
use twilight_http::client::ClientBuilder as HttpClientBuilder;
use twilight_util::link::webhook::parse;

pub struct Logging {
	webhook_url: String,
	id: WebhookId,
	token: String,
	channel_id: ChannelId
}

#[inline]
pub fn new(webhook_url: String) -> Logging {
	Logging::new(webhook_url)
}

#[allow(clippy::new_without_default)]
impl Logging {
	pub fn new(webhook_url: String) -> Self {
		Self {
			webhook_url,
			id: WebhookId::new(1).expect("non zero"),
			token: String::new(),
			channel_id: ChannelId::new(1).expect("non zero")
		}
	}
}

impl Logging {
	pub fn send(&self, http: &Arc<HttpClient>, message: String) -> impl Future<Output = MainResult> {
		let http = Arc::clone(http);
		let id = self.id;
		let token = self.token.clone();
		let message = message.clone();

		async move {
			http.execute_webhook(id, &token)
				.content(&message)
				.exec().await?;

			Ok(())
		}
	}
}

#[async_trait]
impl Module for Logging {
	async fn init(&mut self, stuff: &InitStuff) -> InitResult {
		let (id, token) = parse(&self.webhook_url)?;

		self.id = id;
		self.token = token.ok_or("webhook token missing")?.into();

		self.channel_id = stuff.http.webhook(id)
			.exec().await?
			.model().await?
			.channel_id;

		Ok(())
	}

	async fn handle_event(&self, event: Event) -> HandleResult {
		// i add all events manually so that when something is added/removed, i will
		// notice for sure because the build breaks

		match event.event {
			MessageCreate(msg) => {
				if self.channel_id == msg.channel_id { return Ok(()) }
				self.send(&event.http, format!("message: {}", msg.content)).await?;
			}

			MessageUpdate(msg) => {
				if self.channel_id == msg.channel_id { return Ok(()) }

				if let Some(content) = msg.content {
					self.send(&event.http, format!("message updated: {}", content)).await?;
				} else {
					self.send(&event.http, "message updated: content not updated".into()).await?;
				}
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
			MessageUpdate(_) => {}
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
