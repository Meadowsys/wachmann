use futures::Future;
use twilight_gateway::Event as GatewayEvent;
use twilight_http::Client as HttpClient;

#[derive(Debug, Clone)]
pub struct Event {
	pub shard_id: u64,
	pub event: GatewayEvent,
	pub http: HttpClient
}

pub fn create_spawner<F, C>(event: Event) -> impl Fn(C) -> ()
where
	C: Fn(Event) -> F,
	F: Future + Send + 'static,
	F::Output: Send + 'static
{
	move |f| {
		tokio::spawn(f(event.clone()));
	}
}
