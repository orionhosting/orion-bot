import "dotenv/config";
import { GatewayIntentBits, ActivityType } from "discord.js";
import { Pterobot, Localizations } from "@/structures/index";
import { config } from "@/config/index";

Localizations.initialize({
    onLog: (message: string) => console.log(message),
    onError: (message: string) => console.error(message),
});

const client = new Pterobot({
    intents: GatewayIntentBits.Guilds | GatewayIntentBits.GuildExpressions,
    presence: {
        status: "online",
        activities: [
            {
                name: "High-performance hosting",
                state: config.domain,
                type: ActivityType.Custom,
            },
        ],
    },
});

client.start();
