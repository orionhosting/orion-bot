import type { APIEmbed } from "discord.js";
import { Command, CommandCategory, type CommandData, type SlashHandlerContext } from "@/structures/index";

export default class extends Command {
    public static override readonly data: CommandData = {
        name: "botinfo",
        category: CommandCategory.Informations,
    };

    public override async handleSlash({ lang, interaction }: SlashHandlerContext) {
        const embed: APIEmbed = {
            color: this.colors.primary.int,
            title: this.config.hostingName,
            thumbnail: {
                url: this.client.user.displayAvatarURL(),
            },
            fields: [
                {
                    name: `📊 • ${lang.t("stats.title")}`,
                    value:
                        `> ${lang.t("stats.latency")}: ${this.client.ws.ping}ms` +
                        `\n> ${lang.t("stats.creation")}: <t:${Math.floor(this.client.user.createdTimestamp / 1000)}:F>` +
                        `\n> ${lang.t("stats.uptime")}: <t:${Math.floor(Date.now() / 1000) - Math.floor(process.uptime())}:R>`,
                },
                {
                    name: `🛡 • ${lang.t("admins.title")}`,
                    value: this.config.teamMembers.map(m => `> <@${m.id}> (\`${m.username}\`)`).join("\n"),
                },
            ],
        };

        await interaction.reply({ embeds: [embed] });
    }
}
