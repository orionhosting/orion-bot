import { MessageCreateOptions, Routes, type APIEmbed, type WebhookMessageCreateOptions } from "discord.js";
import { colors } from "@/config/index";
import type { OrionBot } from "./bot";

export default class RemoteLogger {
    public constructor(public readonly client: OrionBot) {}

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
        const emoji = type === "log" ? "🟢" : type === "warn" ? "🟠" : "🔴";
        await this.sendMessage(`${emoji} ${message}`);
    }

    public async sendMessage(message: string | WebhookMessageCreateOptions) {
        if (!this.client.readyTimestamp) {
            this.client.logger.warn(message, `Unable to send log message because client is not ready: ${message}`);
            return;
        }

        if (!process.env.LOGS_WEBHOOK_ID || !process.env.LOGS_WEBHOOK_TOKEN) {
            throw new Error("Logs webhook is undefined");
        }

        if (process.env.NODE_ENV !== "production") {
            this.client.logger.info(message);
            return;
        }

        try {
            let messagePayload: MessageCreateOptions;

            if (typeof message === "string") {
                messagePayload = {
                    content: `\`[LOG]\` ${message}`.slice(0, 2000),
                    allowedMentions: { parse: [] },
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
            this.client.logger.error(err, "RemoteLogger error");
        }
    }
}
