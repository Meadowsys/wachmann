import "source-map-support/register";
import "dotenv/config";

import fs from "fs";
import net from "net";
import path from "path";
import { arangojs } from "arangojs";
import { get_env } from "./env";

// everything prints to console.error, console.log only used to print server address
let orig_console_log = console.log;
console.log = console.error;

let env = get_env();

let db = arangojs({
	url: env.arango_url,
	databaseName: env.arango_database,
	auth: {
		username: env.arango_user,
		password: env.arango_password
	}
});

let i = 0;
function sock_path_gen() {
	return path.resolve(`db-${i++}.sock`);
}
let sock_path = sock_path_gen();
while (fs.existsSync(sock_path)) sock_path = sock_path_gen();

const server = net.createServer();
server.on("connection", handle_connection);

await new Promise<void>(r => server.listen(sock_path, r))
	.then(() => {
		orig_console_log(sock_path)
		orig_console_log = console.error; // its gone forever muehehhe
	});

server.once("close", () => console.log("server closed"));
["SIGINT", "SIGTERM", "exit"].forEach(s => process.on(s, () => {
	server.close(() => {});
}));

function handle_connection() {
	// todo lol
}
