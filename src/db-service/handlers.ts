import net from "net";
import { client_messages } from "./client-messages";
import type { GetTestDataMessage, PutTestDataMessage } from "./client-messages";
import type { ServerMessages } from "./server-messages";
import type { Database } from "arangojs";

export async function create_handle_data(
	connection: net.Socket,
	db: Database
) {
	const newline_buf = Buffer.from("\n");
	let residual_data = "";

	let test_collection = db.collection("test");
	if (!await test_collection.exists()) await test_collection.create();

	return { write, handle_data };

	function write(data: ServerMessages): Promise<void> {
		return new Promise((res, rej) => {
			connection.write(
				Buffer.concat([Buffer.from(JSON.stringify(data)), newline_buf]),
				err => err ? rej(err) : res()
			);
		});
	}

	function handle_data(data?: Buffer) {
		if (data) residual_data += data.toString();

		while (true) {
			console.log(residual_data);
			let next_newline_index = residual_data.indexOf("\n");
			if (next_newline_index === -1) return;

			let unparsed_data = residual_data.substring(0, next_newline_index);
			residual_data = residual_data.substring(next_newline_index + 1).trimStart();
			console.log(`processing ${unparsed_data}`);

			let parse_result = client_messages.safeParse(JSON.parse(unparsed_data));
			if (!parse_result.success) {
				write({
					message: "error",
					error: JSON.stringify(parse_result.error.format(), null, 3)
				});
				continue;
			}

			let data = parse_result.data;

			if (data.message === "get_test_data") {
				return handle_get_test_data(data);
			}

			if (data.message === "put_test_data") {
				return handle_put_test_data(data);
			}

			// i trust typescript and my ability to program, but also meh why not lol
			write({
				message: "error",
				error: `Unknown message: ${(data as any).message}`
			});
		}
	}

	async function handle_get_test_data(msg: GetTestDataMessage) {
		// test_collection.document(msg.id);
		console.log(JSON.stringify(msg, null, 3));

		write({
			message: "test_data",
			data: "wefiojfweoewfijjwefjiooijoij",
			key: "42"
		});

		handle_data();
	}

	async function handle_put_test_data(msg: PutTestDataMessage) {
		console.log(JSON.stringify(msg, null, 3));

		let document = await test_collection.save(
			{ data: msg.data },
			{ returnNew: true }
		);

		write({
			message: "test_data",
			data: document.new.data,
			key: document._key
		});

		handle_data();
	}
}
