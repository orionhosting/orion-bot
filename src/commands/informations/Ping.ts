import {
    ActionRowBuilder,
    ApplicationIntegrationType,
    ButtonBuilder,
    ButtonStyle,
    ContainerBuilder,
    InteractionContextType,
    MessageFlags,
    SeparatorBuilder,
    SeparatorSpacingSize,
    TextDisplayBuilder,
} from "discord.js";
import { Command, CommandCategory, type CommandData, type CommandHandlerContext } from "@/structures/index";

export default class extends Command {
    public static override readonly data: CommandData = {
        name: "ping",
        category: CommandCategory.Informations,
        integrationTypes: [ApplicationIntegrationType.GuildInstall, ApplicationIntegrationType.UserInstall],
        contexts: [InteractionContextType.Guild, InteractionContextType.BotDM, InteractionContextType.PrivateChannel],
    };

    public override async handleCommand({ lang, interaction }: CommandHandlerContext) {
        const response = await interaction.reply({
            components: [
                new ContainerBuilder().addTextDisplayComponents(
                    new TextDisplayBuilder().setContent(`${this.emojis.loaderGreen} ${lang.t("pinging")}`),
                ),
            ],
            flags: MessageFlags.IsComponentsV2,
            withResponse: true,
        });

        const discordPing = (response.resource?.message?.createdTimestamp || 0) - interaction.createdTimestamp;
        const domainPing = await this.client.services.status.pingURL(this.config.domainURL);
        const orionAPIPing = await this.client.services.status.pingURL(this.config.apiURL);
        const docsPing = await this.client.services.status.pingURL(this.config.docsURL);
        const panelPing = await this.client.services.status.pingURL(this.config.panelURL);
        const fr1NodePing = await this.client.services.status.pingURL(`http://fr1.${this.config.domain}`);
        const offlineLabel = lang.common("offline").toLowerCase();

        const container = new ContainerBuilder()
            .addTextDisplayComponents(
                new TextDisplayBuilder().setContent(`## ${this.emojis.logo}  ${lang.t("services.title")}`),
            )
            .addTextDisplayComponents(
                new TextDisplayBuilder().setContent(
                    `> ${domainPing ? this.emojis.online : this.emojis.dnd} Orion Website: \`${domainPing ? `${domainPing}ms` : offlineLabel}\`` +
                        `\n> ${panelPing ? this.emojis.online : this.emojis.dnd} Orion Panel: \`${panelPing ? `${panelPing}ms` : offlineLabel}\`` +
                        `\n> ${docsPing ? this.emojis.online : this.emojis.dnd} Orion Docs: \`${docsPing ? `${docsPing}ms` : offlineLabel}\`` +
                        `\n> ${orionAPIPing ? this.emojis.online : this.emojis.dnd} Orion API: \`${orionAPIPing ? `${orionAPIPing}ms` : offlineLabel}\`` +
                        `\n> ${fr1NodePing ? this.emojis.online : this.emojis.dnd} Node fr1: \`${fr1NodePing ? `${fr1NodePing}ms` : offlineLabel}\``,
                ),
            )
            .addTextDisplayComponents(new TextDisplayBuilder().setContent(lang.t("services.more")))
            .addActionRowComponents(
                new ActionRowBuilder<ButtonBuilder>().setComponents(
                    new ButtonBuilder()
                        .setStyle(ButtonStyle.Link)
                        .setEmoji({ id: this.emojis.propertyId })
                        .setLabel(lang.common("status_page"))
                        .setURL(this.config.statusURL),
                ),
            )
            .addSeparatorComponents(new SeparatorBuilder().setDivider(true).setSpacing(SeparatorSpacingSize.Large))
            .addTextDisplayComponents(
                new TextDisplayBuilder().setContent(`## ${this.emojis.discord}  ${lang.t("bot.title")}`),
            )
            .addTextDisplayComponents(
                new TextDisplayBuilder().setContent(
                    `> Discord Gateway: \`${this.client.ws.ping}ms\`` + `\n> Discord API: \`${discordPing}ms\``,
                ),
            );

        await interaction.editReply({
            content: null,
            components: [container],
            flags: MessageFlags.IsComponentsV2,
            allowedMentions: { parse: [] },
        });
    }
}
