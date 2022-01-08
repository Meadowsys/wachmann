// structs and things for messages sent by the client to the server

import { z, literal, object, string, union } from "zod";

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
// export const client_messages = union([
// 	save_message_message
// ]);
export const client_messages = save_message_message;
