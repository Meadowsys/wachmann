import net from "net";
import { client_messages } from "./client-messages";
import { message_message } from "./server-messages";
import type { Database } from "arangojs";
import type { SaveMessageMessage, GetMessageMessage } from "./client-messages";
import type { ServerMessages } from "./server-messages";

export async function create_handle_data(
	connection: net.Socket,
	db: Database
) {
	const newline_buf = Buffer.from("\n");
	let residual_data = "";

	let test_collection = db.collection("test");
	if (!await test_collection.exists()) await test_collection.create();

	let messages_collection = db.collection("messages");
	if (!await messages_collection.exists()) await messages_collection.create();

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
			let next_newline_index = residual_data.indexOf("\n");
			if (next_newline_index === -1) return;

			let unparsed_data = residual_data.substring(0, next_newline_index);
			residual_data = residual_data.substring(next_newline_index + 1).trimStart();

			let parse_result = client_messages.safeParse(JSON.parse(unparsed_data));
			if (!parse_result.success) {
				write({
					message: "error",
					error: JSON.stringify(parse_result.error.format(), null, 3)
				});
				continue;
			}

			let data = parse_result.data;

			if (data.message === "save_message") {
				return handle_save_message(data);
			}
			if (data.message = "get_message") {
				return handle_get_message(data);
			}

			// i trust typescript and my ability to program, but also meh why not lol
			write({
				message: "error",
				error: `Unknown message: ${(data as any).message}`
			});
		}
	}

	async function handle_save_message(query: SaveMessageMessage) {
		// @ts-expect-error
		delete query.message; delete query.id;
		await messages_collection.save(query);
		write({ message: "ok" });
	}

	async function handle_get_message(query: GetMessageMessage) {
		let msg = await messages_collection.document(
			{ _key: query.id },
			{ graceful: true }
		);

		if (!msg) return void write({ message: "no_message" });

		let msg_parse_result = message_message.safeParse({
			message: "message",
			...msg,
			id: msg._key
		});
		if (!msg_parse_result.success) return void write({
			message: "error",
			error: JSON.stringify(msg_parse_result.error.format())
		});

		write(msg_parse_result.data);
	}
}
