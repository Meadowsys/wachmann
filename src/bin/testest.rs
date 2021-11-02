#![allow(unused)]

use std::env::var;
use std::error::Error;
use std::time::Duration;
use twilight_embed_builder::{ EmbedBuilder, EmbedAuthorBuilder, EmbedFieldBuilder, EmbedFooterBuilder, ImageSource, EmbedError, EmbedErrorType };
use twilight_http::Client as HttpClient;
use twilight_model::id::ChannelId;
use twilight_http::Client;
use chrono::{ DateTime };

type TheResult = Result<(), Box<dyn Error + Send + Sync>>;

fn main() -> TheResult {
	let rt = tokio::runtime::Builder::new_multi_thread()
		.enable_all()
		.worker_threads(2)
		.max_blocking_threads(32)
		.thread_keep_alive(Duration::from_secs(60))
		.build()?;

	rt.block_on(async_main())?;
	rt.shutdown_timeout(Duration::from_secs(60));

	Ok(())
}

async fn async_main() -> TheResult {
	let http = HttpClient::builder()
		.token(var("BOT_TOKEN").unwrap())
		.build();
	let channel_id = ChannelId(834298528741326869);

	let embed = EmbedBuilder::new()
		// .author(EmbedAuthorBuilder::new()
		// 	.name("author")
		// 	.icon_url(ImageSource::url("https://cdn.discordapp.com/avatars/849403340193398805/eae1bbdbc47a2e5683a5f39a442c0560.png")?)
		// 	.build()
		// )
		.title("Message edited")
		.color(0xff9f32)
		// .description("description")
		// .footer(EmbedFooterBuilder::new("a footer"))
		// .field(EmbedFieldBuilder::new("image url source btw but inlined", "https://derpibooru.org/images/2624434")
		// 	.inline()
		// 	.build()
		// )
		// .image(ImageSource::url("https://derpicdn.net/img/2021/5/29/2624434/large.jpg")?)
		// .thumbnail(ImageSource::url("https://derpicdn.net/img/2021/5/29/2624434/large.jpg")?)
		.field(EmbedFieldBuilder::new("field title lol", "field description lol")
			.inline()
			.build()
		)
		.timestamp(chrono::Utc::now().to_rfc3339())
		.build()?;

	http.create_message(channel_id)
		.embeds(&[embed])?
		.exec().await?;
	// http.create_message(channel_id)
	// 	.content("wait not embed, i mean field aa")?
	// 	.exec().await?;

	Ok(())
}
