import { z } from "zod";

export const envSchema = z.object({
    NODE_ENV: z.union([z.literal("production"), z.literal("development"), z.literal("test")]),
    PORT: z.string(),
    DISCORD_ID: z.string(),
    DISCORD_TOKEN: z.string(),
    LOGS_WEBHOOK_ID: z.string(),
    LOGS_WEBHOOK_TOKEN: z.string(),
    PTERO_API_KEY: z.string(),
    MONGODB_URI: z.string(),
});
