import { z, literal, object, string, union } from "zod";

export const config_parser = object({
	guild: string(),
	main_log_channel: string().optional(),
	main_webhook_id: string().optional(),
	main_webhook_token: string().optional()
});
