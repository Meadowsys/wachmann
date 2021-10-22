use std::error::Error;

/// generates an inlineable function that returns the provided ids. in the macro
/// call, you provide a production id and a development id, and it will return
/// the appropriate one depending on the build mode (debug or release build)
///
/// ```
/// id!(galacon_main -> GuildId, dev 12345, prod 54321);
/// id!(server_support -> ChannelId, dev 44556, prod 37485)
/// let id = galacon_main();
/// ```
///
/// If you want a method for a struct or something to force it to
/// be called on an instance of that struct, you can do
/// that, just add `method` to the beginning of the macro call like so:
/// ```
/// struct H;
/// impl H {
///    id!(method galacon_main -> GuildId, dev 12345, prod 54321);
/// }
///
/// let id = H.galacon_main();
/// ```
///
/// No idea if it can pull values from `self`, that would be interesting but also
/// that is not supported, and not the point of this macro lol
macro_rules! id {
	(name: $name:ident, type: $type:ident, development: $dev:expr, production: $prod:expr) => {
		// development
		#[inline]
		#[cfg(debug_assertions)]
		pub fn $name() -> $type { $type($dev) }

		// production
		#[inline]
		#[cfg(not(debug_assertions))]
		pub fn $name() -> $type { $type($prod) }
	};
	($name:ident -> $type:ident, dev $dev:expr, prod $prod:expr) => {
		id!(name: $name, type: $type, development: $dev, production: $prod);
	};

	(method name: $name:ident, type: $type:ident, development: $dev:expr, production: $prod:expr) => {
		// development
		#[inline]
		#[cfg(debug_assertions)]
		pub fn $name(&self) -> $type { $type($dev) }

		// production
		#[inline]
		#[cfg(not(debug_assertions))]
		pub fn $name(&self) -> $type { $type($prod) }
	};
	(method $name:ident -> $type:ident, dev $dev:expr, prod $prod:expr) => {
		id!(method name: $name, type: $type, development: $dev, production: $prod);
	};
}

#[cfg(debug_assertions)]
fn init_dotenv() {
	if let Err(e) = dotenv::dotenv() {
		eprintln!("dotenv failed to initialise: {}", e);
	} else {
		eprintln!("initialised dotenv successfully");
	}
}

#[cfg(not(debug_assertions))]
fn init_dotenv() {}

pub(crate) struct Env {
	token: String
}

impl Env {
	pub fn get_env() -> Result<Env, Box<dyn Error + Send + Sync>> {
		use std::env::var;

		init_dotenv();

		let token = var("TOKEN")
			.or_else(|_| var("BOT_TOKEN"))
		?;

		Ok(Env { token })
	}

	pub fn token(&self) -> &str {
		&self.token
	}

	#[inline]
	#[cfg(debug_assertions)]
	pub fn is_production(&self) -> bool { false }

	#[inline]
	#[cfg(not(debug_assertions))]
	pub fn is_production(&self) -> bool { true }

	#[inline]
	#[cfg(debug_assertions)]
	pub fn is_development(&self) -> bool { true }

	#[inline]
	#[cfg(not(debug_assertions))]
	pub fn is_development(&self) -> bool { false }
}
