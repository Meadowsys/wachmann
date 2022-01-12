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
#[allow(unused_macros)]
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
