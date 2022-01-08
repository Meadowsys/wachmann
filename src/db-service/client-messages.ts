// structs and things for messages sent by the client to the server

import { z, literal, object, string, union } from "zod";

export type PutTestDataMessage = z.infer<typeof put_test_data_message>;
export const put_test_data_message = object({
	message: literal("put_test_data"),
	data: string()
});

export type GetTestDataMessage = z.infer<typeof get_test_data_message>;
export const get_test_data_message = object({
	message: literal("get_test_data"),
	id: string()
});

export type SaveMessageMessage = z.infer<typeof save_message_message>;
export const save_message_message = object({
	message: literal("save_message"),
	id: string(),
	channel_id: string(),
	author_id: string(),
	content: string(),
	attachment_urls: string().array()
}).transform(msg => ({ ...msg, _key: msg.id }));

export type ClientMessages = z.infer<typeof client_messages>;
export const client_messages = union([
	put_test_data_message,
	get_test_data_message,
	save_message_message
]);
