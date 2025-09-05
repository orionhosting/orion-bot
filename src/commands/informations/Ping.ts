import type { APIEmbed } from "discord.js";
import { Command, CommandCategory, type CommandData, type SlashHandlerContext } from "@/structures/index";

export default class extends Command {
    public static override readonly data: CommandData = {
        name: "ping",
        category: CommandCategory.Informations,
    };

    public override async handleSlash({ lang, interaction }: SlashHandlerContext) {
        const response = await interaction.deferReply({ withResponse: true });

        const ping = (response.resource?.message?.createdTimestamp || 0) - interaction.createdTimestamp;

        const embed: APIEmbed = {
            color: this.colors.primary.int,
            fields: [
                {
                    name: lang.t("pings.discord_api"),
                    value: `🔵 ${this.client.ws.ping}ms`,
                    inline: true,
                },
                {
                    name: lang.t("pings.message"),
                    value: `🔵 ${ping}ms`,
                    inline: true,
                },
            ],
        };

        await interaction.editReply({ embeds: [embed] });
    }
}
