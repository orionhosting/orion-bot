import {
    ActionRowBuilder,
    ApplicationIntegrationType,
    ButtonBuilder,
    ButtonStyle,
    ContainerBuilder,
    InteractionContextType,
    MessageFlags,
    TextDisplayBuilder,
} from "discord.js";
import { OrionAPIError } from "@/modules/orion-api";
import { Command, CommandCategory, type CommandData, type CommandHandlerContext } from "@/structures/index";

export default class extends Command {
    public static override readonly data: CommandData = {
        name: "account",
        category: CommandCategory.Informations,
        integrationTypes: [ApplicationIntegrationType.GuildInstall, ApplicationIntegrationType.UserInstall],
        contexts: [InteractionContextType.Guild, InteractionContextType.BotDM, InteractionContextType.PrivateChannel],
    };

    public override async handleCommand({ lang, interaction }: CommandHandlerContext) {
        try {
            const user = await this.client.orionAPI.getUser(interaction.user.id);

            const container = new ContainerBuilder()
                .setAccentColor(this.colors.primary.int)
                .addTextDisplayComponents(
                    new TextDisplayBuilder().setContent(
                        `## ${this.emojis.logo} Orion - ${lang.t("card.title.your_account")}`,
                    ),
                )
                .addTextDisplayComponents(
                    new TextDisplayBuilder().setContent(
                        lang.t("card.content.credits", { amount: user.credits.toString() }),
                    ),
                )
                .addActionRowComponents(
                    new ActionRowBuilder<ButtonBuilder>().setComponents(
                        new ButtonBuilder()
                            .setStyle(ButtonStyle.Link)
                            .setLabel(lang.common("dashboard"))
                            .setURL(`${this.config.domainURL}/dashboard`),
                        new ButtonBuilder()
                            .setStyle(ButtonStyle.Link)
                            .setLabel(lang.common("panel"))
                            .setURL(this.config.panelURL),
                    ),
                );

            await interaction.reply({
                components: [container],
                flags: MessageFlags.IsComponentsV2,
                allowedMentions: { parse: [] },
            });
        } catch (err) {
            if (!(err instanceof OrionAPIError) || err.status !== 404) {
                throw err;
            }

            const container = new ContainerBuilder()
                .setAccentColor(this.colors.primary.int)
                .addTextDisplayComponents(new TextDisplayBuilder().setContent(`## ${this.emojis.logo} Orion`))
                .addTextDisplayComponents(new TextDisplayBuilder().setContent(lang.t("no_account.content.guide")))
                .addActionRowComponents(
                    new ActionRowBuilder<ButtonBuilder>().setComponents(
                        new ButtonBuilder()
                            .setStyle(ButtonStyle.Link)
                            .setLabel(lang.t("no_account.buttons.create_account"))
                            .setURL(`${this.config.domainURL}/dashboard`),
                    ),
                );

            await interaction.reply({
                components: [container],
                flags: MessageFlags.IsComponentsV2,
                allowedMentions: { parse: [] },
            });
        }
    }
}
