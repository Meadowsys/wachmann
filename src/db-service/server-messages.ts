// structs and things for messages sent by the server to the client

import { z, literal, object, string } from "zod";

export type ReadyMessage = z.infer<typeof ready_message>;
export const ready_message = object({
	message: literal("ready"),
	id: string().length(5)
});
