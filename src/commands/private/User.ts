import { ApplicationCommandOptionType, ContainerBuilder, MessageFlags, TextDisplayBuilder } from "discord.js";
import { config } from "@/config";
import { snowflake } from "@/lib/snowflake";
import { OrionAPIError } from "@/modules/orion-api";
import { Command, CommandCategory, type CommandData, type CommandHandlerContext } from "@/structures/index";

export default class extends Command {
    public static override readonly data: CommandData = {
        name: "user",
        category: CommandCategory.Private,
        description: "Get a user's Orion account info",
        guilds: [config.supportGuildId],
        ownerOnly: true,
        options: [
            {
                name: "user",
                type: ApplicationCommandOptionType.User,
                description: "The user to look up",
                required: true,
            },
        ],
    };

    public override async handleCommand({ interaction }: CommandHandlerContext) {
        const user = interaction.options.getUser("user", true);

        await interaction.deferReply({ flags: MessageFlags.Ephemeral });

        let orionUser;
        try {
            orionUser = await this.client.orionAPI.getUser(user.id);
        } catch (err) {
            if (err instanceof OrionAPIError && err.status === 404) {
                await interaction.editReply(`${this.emojis.warn} This user does not have an Orion account`);
                return;
            }

            throw err;
        }

        const container = new ContainerBuilder()
            .setAccentColor(this.colors.primary.int)
            .addTextDisplayComponents(new TextDisplayBuilder().setContent(`## ${this.emojis.logo} Orion Account`))
            .addTextDisplayComponents(
                new TextDisplayBuilder().setContent(
                    `> Discord ID: \`${orionUser.discord_id}\`` +
                        `\n> Orion ID: \`${orionUser.id}\`` +
                        `\n> Panel ID: \`${orionUser.panel_id}\``,
                ),
            )
            .addTextDisplayComponents(
                new TextDisplayBuilder().setContent(
                    `\n> Username: \`${orionUser.username}\`` + `\n> Credits: \`${orionUser.credits}\``,
                ),
            )
            .addTextDisplayComponents(
                new TextDisplayBuilder().setContent(
                    `\n> Referral code: \`${orionUser.referral_code}\`` +
                        `\n> Referral usage: \`${orionUser.referral_usage}\`` +
                        `\n> Referral reward: \`${orionUser.referral_gains} credits\``,
                ),
            )
            .addTextDisplayComponents(
                new TextDisplayBuilder().setContent(
                    `\n> Last dashboard login: <t:${Math.floor(orionUser.last_login_at / 1000)}:R>` +
                        `\n> Account creation: <t:${Math.floor(snowflake.timestampFrom(orionUser.id) / 1000)}:R>`,
                ),
            );

        await interaction.editReply({
            components: [container],
            flags: MessageFlags.IsComponentsV2,
            allowedMentions: { parse: [] },
        });
    }
}
