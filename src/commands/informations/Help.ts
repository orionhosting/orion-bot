import {
    ActionRowBuilder,
    ContainerBuilder,
    MediaGalleryBuilder,
    MediaGalleryItemBuilder,
    TextDisplayBuilder,
    MessageFlags,
    ButtonBuilder,
    ButtonStyle,
    SeparatorBuilder,
    SeparatorSpacingSize,
    ApplicationIntegrationType,
    InteractionContextType,
} from "discord.js";
import { Command, CommandCategory, type CommandData, type CommandHandlerContext } from "@/structures/index";

export default class extends Command {
    public static override readonly data: CommandData = {
        name: "help",
        category: CommandCategory.Informations,
        integrationTypes: [ApplicationIntegrationType.GuildInstall, ApplicationIntegrationType.UserInstall],
        contexts: [InteractionContextType.Guild, InteractionContextType.BotDM, InteractionContextType.PrivateChannel],
    };

    public override async handleCommand({ lang, interaction }: CommandHandlerContext) {
        const container = new ContainerBuilder()
            .setAccentColor(this.colors.primary.int)
            .addTextDisplayComponents(
                new TextDisplayBuilder().setContent(`## ${this.emojis.logo} Orion - ${lang.t("title")}`),
            )
            .addMediaGalleryComponents(
                new MediaGalleryBuilder().addItems(new MediaGalleryItemBuilder().setURL(this.config.bannerURL)),
            )
            .addTextDisplayComponents(
                new TextDisplayBuilder().setContent(
                    `### ${lang.t("hosting.title")}` + `\n> ${lang.t("hosting.description")}`,
                ),
            )
            .addActionRowComponents(
                new ActionRowBuilder<ButtonBuilder>().setComponents(
                    new ButtonBuilder()
                        .setEmoji({ id: this.emojis.logoId })
                        .setStyle(ButtonStyle.Link)
                        .setLabel(lang.t("hosting.buttons.create_free_server"))
                        .setURL(`${this.config.domainURL}/dashboard`),
                ),
            )
            .addSeparatorComponents(new SeparatorBuilder().setDivider(false).setSpacing(SeparatorSpacingSize.Small))
            .addTextDisplayComponents(
                new TextDisplayBuilder().setContent(
                    `### ${lang.t("services.title")}` +
                        `\n> ${lang.t("services.website")}: ${this.config.domainURL}` +
                        `\n> ${lang.common("dashboard")}: ${this.config.domainURL}/dashboard` +
                        `\n> ${lang.common("panel")}: ${this.config.panelURL}` +
                        `\n> ${lang.common("documentation")}: ${this.config.docsURL}` +
                        `\n> ${lang.common("status")}: ${this.config.statusURL}`,
                ),
            )
            .addActionRowComponents(
                new ActionRowBuilder<ButtonBuilder>().setComponents(
                    new ButtonBuilder()
                        .setEmoji({ id: this.emojis.discordId })
                        .setStyle(ButtonStyle.Link)
                        .setLabel(lang.t("services.buttons.join_support"))
                        .setURL(this.config.supportInvite),
                    new ButtonBuilder()
                        .setEmoji({ id: this.emojis.logoId })
                        .setStyle(ButtonStyle.Link)
                        .setLabel(lang.t("services.buttons.add_bot"))
                        .setURL(this.config.botInvite),
                ),
            );

        await interaction.reply({
            components: [container],
            flags: MessageFlags.IsComponentsV2,
            allowedMentions: { parse: [] },
        });
    }
}
