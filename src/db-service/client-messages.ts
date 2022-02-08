// structs and things for messages sent by the client to the server

import { z, literal, object, string, union } from "zod";

export type SaveMessage = z.infer<typeof save_message_parser>;
export const save_message_parser = object({
	message: literal("save_message"),
	id: string(),
	channel_id: string(),
	author_id: string(),
	content: string(),
	attachment_urls: string().array()
}).transform(msg => ({ ...msg, _key: msg.id }));

export type GetMessage = z.infer<typeof get_message_parser>;
export const get_message_parser = object({
	message: literal("get_message"),
	id: string()
});

export type SaveUser = z.infer<typeof save_user_parser>;
export const save_user_parser = object({
	message: literal("save_user"),
	id: string(),
	name: string(),
	discriminator: string().refine(discriminator => /^\d{4}$/.test(discriminator)),
	avatar_url: string().url()
}).transform(msg => ({ ...msg, _key: msg.id }));

export type GetUser = z.infer<typeof get_user_parser>;
export const get_user_parser = object({
	message: literal("get_user"),
	id: string()
});

export type ClientMessages = z.infer<typeof client_messages>;
export const client_messages = union([
	save_message_parser,
	get_message_parser,
	save_user_parser,
	get_user_parser
]);
