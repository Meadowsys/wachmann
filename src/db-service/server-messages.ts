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

export type TestDataMessage = z.infer<typeof test_data_message>;
export const test_data_message = object({
	message: literal("test_data"),
	key: string(),
	data: string()
});

export type ErrorMessage = z.infer<typeof error_message>;
export const error_message = object({
	message: literal("error"),
	error: string()
});

export type ServerMessages = z.infer<typeof server_messages>;
export const server_messages = union([
	ready_message,
	ok_message,
	test_data_message,
	error_message
]);
