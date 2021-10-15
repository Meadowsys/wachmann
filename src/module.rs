use futures::Future;
use twilight_gateway::Event as GatewayEvent;
use twilight_http::Client as HttpClient;

#[derive(Debug, Clone)]
pub struct Event {
	pub shard_id: u64,
	pub event: GatewayEvent,
	pub http: HttpClient
}
