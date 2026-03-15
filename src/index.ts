import "dotenv/config";
import { GatewayIntentBits, ActivityType } from "discord.js";
import { config } from "@/config/index";
import { OrionBot, Localizations } from "@/structures/index";
import { envSchema } from "./validation/env";

try {
    envSchema.parse(process.env);
} catch (error) {
    console.error("Invalid environment variables", error);
    process.exit(1);
}

Localizations.initialize({
    onLog: (message: string) => console.log(message),
    onError: (message: string) => console.error(message),
});

const client = new OrionBot({
    intents:
        GatewayIntentBits.Guilds |
        GatewayIntentBits.GuildExpressions |
        GatewayIntentBits.GuildMessages |
        GatewayIntentBits.GuildMembers |
        GatewayIntentBits.MessageContent,
    presence: {
        status: "online",
        activities: [
            {
                name: "Free hosting!",
                state: config.domain,
                type: ActivityType.Custom,
            },
        ],
    },
});

client.start();
