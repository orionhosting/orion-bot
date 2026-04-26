import assert from "node:assert";
import { Time } from "@voctal/duration";
import {
    ActionRowBuilder,
    ButtonBuilder,
    ButtonStyle,
    ContainerBuilder,
    GuildTextBasedChannel,
    MediaGalleryBuilder,
    MediaGalleryItemBuilder,
    MessageFlags,
    SeparatorBuilder,
    SeparatorSpacingSize,
    TextDisplayBuilder,
} from "discord.js";
import type { OrionBot } from "@/structures/index";
import { Service } from "@/structures/services/service";

export class StatusService extends Service {
    public override readonly data = {
        name: "status",
    };

    public readonly updateInterval = Time.Minute;

    /**
     * If the status channel was already fetched since last startup.
     */
    private fetched = false;

    public constructor(client: OrionBot) {
        super(client);
    }

    public override onReady() {
        if (process.env.NODE_ENV !== "production") return;

        setInterval(async () => {
            try {
                await this.tick();
            } catch (err) {
                this.client.monitor.captureException(err, "Status update");
            }
        }, this.updateInterval);
    }

    /**
     * Update the status message.
     */
    private async tick() {
        const guild = this.client.getSupportGuild();
        const channel = guild.channels.cache.get(this.config.statusChannelId);
        assert(channel, "missing status channel");
        assert(channel.isTextBased(), "status channel is not text based");

        const statusMessage = await this.getStatusMessage(channel);

        let status;
        try {
            status = await this.client.orionAPI.getStatus();
        } catch (err) {
            this.client.logger.debug(err, "Could not fetch Orion API during status update");
        }

        const domainPing = await this.pingURL(this.config.domainURL);
        const orionAPIPing = await this.pingURL(this.config.apiURL);
        const orionBotPing = await this.pingURL(this.config.botAPIURL);
        const docsPing = await this.pingURL(this.config.docsURL);
        const panelPing = await this.pingURL(this.config.panelURL);
        const fr1NodePing = await this.pingURL(`http://fr1.${this.config.domain}:8080`);

        const container = new ContainerBuilder()
            .setAccentColor(this.colors.primary.int)
            .addTextDisplayComponents(new TextDisplayBuilder().setContent(`## ${this.emojis.logo} Services Status`))
            .addMediaGalleryComponents(
                new MediaGalleryBuilder().addItems(new MediaGalleryItemBuilder().setURL(this.config.bannerURL)),
            )
            .addTextDisplayComponents(
                new TextDisplayBuilder().setContent(
                    `> ${domainPing ? this.emojis.online : this.emojis.dnd} Orion Website: \`${domainPing ? `${domainPing}ms` : "offline"}\`` +
                        `\n> ${panelPing ? this.emojis.online : this.emojis.dnd} Orion Panel: \`${panelPing ? `${panelPing}ms` : "offline"}\`` +
                        `\n> ${docsPing ? this.emojis.online : this.emojis.dnd} Orion Docs: \`${docsPing ? `${docsPing}ms` : "offline"}\`` +
                        `\n> ${orionBotPing ? this.emojis.online : this.emojis.dnd} Orion Bot: \`${orionBotPing ? `${orionBotPing}ms` : "offline"}\`` +
                        `\n> ${orionAPIPing ? this.emojis.online : this.emojis.dnd} Orion API: \`${orionAPIPing ? `${orionAPIPing}ms` : "offline"}\`` +
                        `\n> ${fr1NodePing ? this.emojis.online : this.emojis.dnd} Node fr1: \`${fr1NodePing ? `${fr1NodePing}ms` : "offline"}\``,
                ),
            )
            .addTextDisplayComponents(
                new TextDisplayBuilder().setContent(
                    `> Users: \`${status?.user_count || "unknown"}\`` +
                        `\n> Servers: \`${status?.server_count || "unknown"}\``,
                ),
            )
            .addTextDisplayComponents(
                new TextDisplayBuilder().setContent(
                    `> Next update <t:${Math.floor((Date.now() + this.updateInterval) / 1000)}:R>`,
                ),
            )
            .addSeparatorComponents(new SeparatorBuilder().setDivider(false).setSpacing(SeparatorSpacingSize.Large))
            .addTextDisplayComponents(
                new TextDisplayBuilder().setContent(`Use the status page for more details about our services.`),
            )
            .addActionRowComponents(
                new ActionRowBuilder<ButtonBuilder>().setComponents(
                    new ButtonBuilder()
                        .setStyle(ButtonStyle.Link)
                        .setEmoji({ id: this.emojis.propertyId })
                        .setLabel("Status Page")
                        .setURL(this.config.statusURL),
                ),
            );

        await statusMessage.edit({ components: [container], allowedMentions: { parse: [] } });
    }

    /**
     * Get the status message to edit.
     */
    private async getStatusMessage(channel: GuildTextBasedChannel) {
        const history = this.fetched ? channel.messages.cache : await channel.messages.fetch({ limit: 15 });

        let message = history.find(
            m => m.author.id === this.client.user.id && m.flags.has(MessageFlags.IsComponentsV2),
        );

        if (!message) {
            message = await channel.send({
                flags: [MessageFlags.IsComponentsV2],
                components: [
                    new ContainerBuilder().addTextDisplayComponents(
                        new TextDisplayBuilder().setContent(`${this.emojis.loaderGreen} Pinging our services...`),
                    ),
                ],
            });
        }

        return message;
    }

    /**
     * Ping a service URL and get its latency.
     */
    public async pingURL(url: string): Promise<number | null> {
        let start = Date.now();
        try {
            await fetch(url, { method: "HEAD" });
            return Date.now() - start;
        } catch (err) {
            if (err instanceof Error && err.cause instanceof Error && err.cause.message === "other side closed") {
                return Date.now() - start;
            }
            return null;
        }
    }
}
