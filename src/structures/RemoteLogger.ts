import { Routes, type APIEmbed, type WebhookMessageCreateOptions } from "discord.js";
import { colors } from "@/config/index";
import type { Pterobot } from "./Pterobot";

export default class RemoteLogger {
    public constructor(public readonly client: Pterobot) {}

    public async sendLog(message: string | WebhookMessageCreateOptions) {
        await this.sendMessage(message);
    }

    public async sendWarning(warning: string | Error) {
        warning =
            typeof warning === "string"
                ? warning
                : warning.stack
                  ? `\`\`\`js\n${warning.stack}\`\`\``
                  : `${warning.name}: ${warning.message}`;

        const embed: APIEmbed = {
            color: colors.orange.int,
            title: `Warning`,
            description: warning.slice(0, 4096),
        };

        await this.sendMessage({ embeds: [embed] });
    }

    public async sendError(origin: string, error: Error) {
        const embed: APIEmbed = {
            color: colors.red.int,
            title: `Error: ${origin}`,
            description: `\`\`\`js\n${error.stack}\`\`\``.slice(0, 4096),
        };

        await this.sendMessage({ embeds: [embed] });
    }

    public async sendInfo(type: "log" | "warn" | "error", message: string) {
        const emoji = type === "log" ? ":blue_circle:" : type === "warn" ? ":orange_circle:" : ":red_circle:";

        await this.sendMessage(`${emoji} ${message}`);
    }

    public async sendMessage(message: string | WebhookMessageCreateOptions) {
        if (!this.client.readyTimestamp) {
            console.warn(`Unable to send log message because client is not ready: ${message}`, JSON.stringify(message));
            return;
        }

        if (!process.env.LOGS_WEBHOOK_ID || !process.env.LOGS_WEBHOOK_TOKEN) {
            throw new Error("Logs webhook is undefined");
        }

        if (process.env.NODE_ENV !== "production") {
            console.error(message);
            return;
        }

        try {
            let messagePayload;

            if (typeof message === "string") {
                messagePayload = {
                    content: `\`[LOG]\` ${message}`.slice(0, 2000),
                };
            } else {
                messagePayload = message;
            }

            await this.client.rest.post(Routes.webhook(process.env.LOGS_WEBHOOK_ID, process.env.LOGS_WEBHOOK_TOKEN), {
                body: messagePayload,
                query: new URLSearchParams("?wait=true"),
                auth: false,
            });
        } catch (err) {
            console.error("RemoteLogger error", err);
        }
    }
}
