import { ActionRowBuilder, ButtonBuilder, ButtonStyle } from "discord.js";
import { Command, CommandCategory, type CommandData, type SlashHandlerContext } from "@/structures/index";

export default class extends Command {
    public static override readonly data: CommandData = {
        name: "account",
        category: CommandCategory.Informations,
    };

    public override async handleSlash({ lang, interaction }: SlashHandlerContext) {
        const row = new ActionRowBuilder<ButtonBuilder>().setComponents(
            new ButtonBuilder().setStyle(ButtonStyle.Link).setLabel(lang.t("my_panel")).setURL(this.config.panelURL),
        );

        await interaction.reply({
            components: [row],
        });
    }
}
