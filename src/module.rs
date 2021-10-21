use futures::Future;
use twilight_gateway::Event as GatewayEvent;
use twilight_http::Client as HttpClient;

#[derive(Debug, Clone)]
pub struct Event {
	pub shard_id: u64,
	pub event: GatewayEvent,
	pub http: HttpClient
}

/// s stands for spawn
pub fn s<F>(event: &Event, f: impl Fn(Event) -> F) -> tokio::task::JoinHandle<F::Output>
where
	F: Future + Send + 'static,
	F::Output: Send + 'static
{
	tokio::spawn(f(event.clone()))
}
