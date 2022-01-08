function throw_no_var_found(...env_var: Array<string>): never {
	throw new Error(`${
		env_var.map(v => `process.env["${v}"]`).join(" or ")
	} not found, please set ${
		env_var.length === 1 ? "it" : "one of them"
	}!`);
}

export function get_env() {
	let node_env = process.env.NODE_ENV;
	if (!node_env || !(node_env === "development" || node_env === "production")) {
		throw "check rollup config, there must be a typo in the define config for node_env";
	}

	let bot_token = process.env.TOKEN ?? process.env.BOT_TOKEN;
	if (!bot_token) throw_no_var_found("TOKEN", "BOT_TOKEN");

	let arango_url = process.env.ARANGO_URL ?? process.env.DB_URL;
	if (!arango_url) throw_no_var_found("ARANGO_URL", "DB_URL");

	let arango_user = process.env.ARANGO_USER ?? process.env.DB_USER;
	if (!arango_user) throw_no_var_found("ARANGO_USER", "DB_USER");

	let arango_password = process.env.ARANGO_PASSWORD ?? process.env.DB_PASSWORD;
	if (!arango_password) throw_no_var_found("ARANGO_PASSWORD", "DB_PASSWORD");

	let arango_database = process.env.ARANGO_DATABASE ?? process.env.DB_DATABASE;
	if (!arango_database) throw_no_var_found("ARANGO_DATABASE", "DB_DATABASE");

	let rv = {
		node_env,
		bot_token,
		arango_url,
		arango_user,
		arango_password,
		arango_database
	};

	// this will error if any of the above are undefined
	// will be stripped out by rollup
	let _typecheck: Record<string, string> = rv;

	return rv;
}
