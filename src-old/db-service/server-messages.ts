// structs and things for messages sent by the server to the client

import { z, literal, object, string, union } from "zod";
import { config_parser as config_struct_parser } from "./config";

export type Ready = z.infer<typeof ready_parser>;
export const ready_parser = object({
	message: literal("ready")
});

export type Ok = z.infer<typeof ok_parser>;
export const ok_parser = object({
	message: literal("ok")
});

export type Error = z.infer<typeof error_parser>;
export const error_parser = object({
	message: literal("error"),
	error: string()
});

export type No = z.infer<typeof no_parser>;
export const no_parser = object({
	message: literal("no")
});

export type Message = z.infer<typeof message_parser>;
export const message_parser = object({
	message: literal("message"),
	id: string(),
	channel_id: string(),
	author_id: string(),
	content: string(),
	attachment_urls: string().array()
});

export type User = z.infer<typeof user_parser>;
export const user_parser = object({
	message: literal("user"),
	id: string(),
	name: string(),
	discriminator: string(),
	avatar_url: string().url()
});

export type Config = z.infer<typeof config_parser>;
export const config_parser = object({
	message: literal("config"),
	config: config_struct_parser
});

export type ServerMessages = z.infer<typeof server_messages>;
export const server_messages = union([
	ready_parser,
	ok_parser,
	error_parser,
	no_parser,
	message_parser,
	user_parser,
	config_parser
]);
