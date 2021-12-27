import net from "net";

const sock_path = "db_service.sock";
const server = net.createServer();
await new Promise<void>(r => server.listen(sock_path, r));
server.on("connection", handle_connection);
["SIGINT", "SIGTERM", "exit"].forEach(s => process.on(s, () => {
	server.close();
}));

let num_connections = 0;
function handle_connection(connection: net.Socket) {
	num_connections += 1;

	function write(data: Buffer | string): Promise<void> {
		return new Promise((res, rej) => {
			connection.write(
				Buffer.concat([Buffer.from(data), Buffer.from("\n")]),
				err => err ? rej(err) : res()
			);
		});
	}

	connection.on("data", d => handle_data(d, write, connection));

	connection.on("error", err => {
		console.error("socket errored!");
		console.error(err);
	});

	connection.on("close", _had_err => {
		num_connections -= 1;

		if (num_connections === 0) {
			server.close();
		}
	});

	write("ready");
}

async function handle_data(
	data: Buffer,
	write: (data: Buffer | string) => void,
	_connection: net.Socket
) {
	write(data);
}
