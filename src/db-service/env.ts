function fatal_error_no_var_found(...env_var: Array<string>): never {
	const map_var = (v: string) => `process.env["${v}"]`;
	const or = " or ";
	const end_phrase = env_var.length === 1 ? "it" : "one of them";
	throw new Error(`${env_var.map(map_var).join(or)} not found, please set ${end_phrase}!`);
}

export type Env = ReturnType<typeof _get_env>;

function _get_env() {
	let node_env = process.env.NODE_ENV;
	if (!node_env || !(node_env === "development" || node_env === "production")) {
		throw new Error(`check rollup config, there must be a typo in the define config for process.env.NODE_ENV`);
	}

	let arango_url = process.env.ARANGO_URL ?? process.env.DB_URL;
	if (!arango_url) fatal_error_no_var_found("ARANGO_URL", "DB_URL");

	let arango_user = process.env.ARANGO_USER ?? process.env.DB_USER;
	if (!arango_user) fatal_error_no_var_found("ARANGO_USER", "DB_USER");

	let arango_password = process.env.ARANGO_PASSWORD ?? process.env.DB_PASSWORD;
	if (!arango_password) fatal_error_no_var_found("ARANGO_PASSWORD", "DB_PASSWORD");

	let arango_database = process.env.ARANGO_DATABASE ?? process.env.DB_DATABASE;
	if (!arango_database) fatal_error_no_var_found("ARANGO_DATABASE", "DB_DATABASE");

	let rv = {
		node_env,
		arango_url,
		arango_user,
		arango_password,
		arango_database
	};

	// this will error if any of the above are undefined (must be defined)
	// will be stripped out by rollup
	let _typecheck: Record<string, string> = rv;

	return rv;
}

let env: Env | undefined = undefined;
export function get_env(): Env {
	if (!env) env = _get_env();
	return env;
}
