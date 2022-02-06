// structs and things for messages sent by the server to the client

import { z, literal, object, string, union } from "zod";

export type ReadyMessage = z.infer<typeof ready_message>;
export const ready_message = object({
	message: literal("ready")
});

export type OkMessage = z.infer<typeof ok_message>;
export const ok_message = object({
	message: literal("ok")
});

export type ErrorMessage = z.infer<typeof error_message>;
export const error_message = object({
	message: literal("error"),
	error: string()
});

export type MessageMessage = z.infer<typeof message_message>;
export const message_message = object({
	message: literal("message"),
	id: string(),
	channel_id: string(),
	author_id: string(),
	content: string(),
	attachment_urls: string().array()
});

export type NoMessageMessage = z.infer<typeof no_message_message>;
export const no_message_message = object({
	message: literal("no_message")
});

export type UserMessage = z.infer<typeof user_message>;
export const user_message = object({
	message: literal("user"),
	id: string(),
	name: string(),
	discriminator: string(),
	avatar_url: string().url()
});

export type NoUserMessage = z.infer<typeof no_user_message>;
export const no_user_message = object({
	message: literal("no_user")
});

export type ServerMessages = z.infer<typeof server_messages>;
export const server_messages = union([
	ready_message,
	ok_message,
	error_message,
	message_message,
	no_message_message,
	user_message,
	no_user_message
]);
