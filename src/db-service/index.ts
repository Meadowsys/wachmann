import "source-map-support/register"
import "dotenv/config";
import fs from "fs";
import net from "net";
import { arangojs } from "arangojs";
import { get_env } from "./env";
import { create_handle_data } from "./handlers";

let env = get_env();

let db = arangojs({
	url: env.arango_url,
	databaseName: env.arango_database,
	auth: {
		username: env.arango_user,
		password: env.arango_password
	}
});

const sock_path = "db_service.sock";
if (fs.existsSync(sock_path)) {
	console.log("Error: `db_service.sock` already in use, database connector will not start up. if no service is running on `db_service.sock`, then `rm db_service.sock` and start this server again");
	process.exit(1);
}

const server = net.createServer();
server.on("connection", handle_connection);
await new Promise<void>(r => server.listen(sock_path, r));
["SIGINT", "SIGTERM", "exit"].forEach(s => process.on(s, () => {
	server.close(err => err ?? console.log("server closed"));
}));
console.log("server started");

let {
	increment_connections,
	decrement_connections
} = create_connection_incrementer();

async function handle_connection(connection: net.Socket) {
	increment_connections();

	let { handle_data, write } = await create_handle_data(connection, db);

	connection.on("data", handle_data);

	connection.on("error", err => {
		console.error(`a socket errored!`);
		console.error(err);
	});

	connection.on("close", _had_err => {
		decrement_connections();
	});

	write({ message: "ready" });
}

function create_connection_incrementer() {
	let num_connections = 0;

	return { increment_connections, decrement_connections, get_connections };

	function increment_connections() {
		num_connections += 1;
		console.log(`connection created, ${num_connections} connection${num_connections === 1 ? "" : "s"}`);
		return num_connections;
	}

	function decrement_connections() {
		num_connections -= 1;
		console.log(`connection closed, ${num_connections} connection${num_connections === 1 ? "" : "s"}`);
		return num_connections;
	}

	function get_connections() {
		return num_connections;
	}
}
