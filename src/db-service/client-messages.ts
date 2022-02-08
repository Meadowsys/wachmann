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

export type GetMessageMessage = z.infer<typeof get_message_message>;
export const get_message_message = object({
	message: literal("get_message"),
	id: string()
});

export type SaveUserMessage = z.infer<typeof save_user_message>;
export const save_user_message = object({
	message: literal("save_user"),
	id: string(),
	name: string(),
	discriminator: string().refine(discriminator => /^\d{4}$/.test(discriminator)),
	avatar_url: string().url()
}).transform(msg => ({ ...msg, _key: msg.id }));

export type GetUserMessage = z.infer<typeof get_user_message>;
export const get_user_message = object({
	message: literal("get_user"),
	id: string()
});

export type ClientMessages = z.infer<typeof client_messages>;
export const client_messages = union([
	save_message_message,
	get_message_message,
	save_user_message,
	get_user_message
]);
