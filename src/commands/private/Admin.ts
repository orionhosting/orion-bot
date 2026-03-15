import { ApplicationCommandOptionType, MessageFlags } from "discord.js";
import { config } from "@/config";
import { CreditTransactionType } from "@/database/enums";
import { OrionAPIError } from "@/modules/orion-api";
import { Command, CommandCategory, type CommandData, type CommandHandlerContext } from "@/structures/index";

export default class extends Command {
    public static override readonly data: CommandData = {
        name: "config",
        category: CommandCategory.Private,
        description: "Configure Orion",
        guilds: [config.supportGuildId],
        ownerOnly: true,
        options: [
            {
                name: "get-available-servers",
                type: ApplicationCommandOptionType.Subcommand,
                description: "Get the available servers count",
            },
            {
                name: "set-available-servers",
                type: ApplicationCommandOptionType.Subcommand,
                description: "Set the available servers count",
                options: [
                    {
                        name: "amount",
                        type: ApplicationCommandOptionType.Integer,
                        description: "The quantity",
                        min_value: 1,
                        max_value: 10000,
                        required: true,
                    },
                ],
            },
            {
                name: "credits-add",
                type: ApplicationCommandOptionType.Subcommand,
                description: "Add credits",
                options: [
                    {
                        name: "user",
                        type: ApplicationCommandOptionType.User,
                        description: "The user",
                        required: true,
                    },
                    {
                        name: "type",
                        type: ApplicationCommandOptionType.Integer,
                        description: "The type",
                        choices: [
                            {
                                name: "The user won a giveaway",
                                value: CreditTransactionType.Giveaway,
                            },
                            {
                                name: "The user made a partnership",
                                value: CreditTransactionType.Partnership,
                            },
                            {
                                name: "Other",
                                value: CreditTransactionType.Custom,
                            },
                        ],
                        required: true,
                    },
                    {
                        name: "amount",
                        type: ApplicationCommandOptionType.Integer,
                        description: "The quantity",
                        min_value: 1,
                        max_value: 10000,
                        required: true,
                    },
                    {
                        name: "reason",
                        type: ApplicationCommandOptionType.String,
                        description: "Only needed when the type is custom",
                        max_length: 500,
                        required: true,
                    },
                ],
            },
        ],
    };

    public override async handleCommand({ interaction }: CommandHandlerContext) {
        await interaction.deferReply({ flags: MessageFlags.Ephemeral });

        switch (interaction.options.getSubcommand(true)) {
            case "get-available-servers": {
                const state = await this.client.orionAPI.getState();

                await interaction.editReply(`Available free servers: ${state.available_free_servers}`);
                return;
            }
            case "set-available-servers": {
                const amount = interaction.options.getInteger("amount", true);

                await this.client.orionAPI.patchState({ available_free_servers: amount });

                await interaction.editReply(`Amount updated. New value: ${amount}`);
                return;
            }
            case "credits-add": {
                const user = interaction.options.getUser("user", true);
                const type: CreditTransactionType = interaction.options.getInteger("type", true);
                const amount = interaction.options.getInteger("amount", true);
                const reason = interaction.options.getString("reason")?.trim();

                if (type === CreditTransactionType.Custom) {
                    if (!reason) {
                        await interaction.editReply("You need to specify the 'reason'. It will be shown to the user.");
                        return;
                    }
                } else if (reason) {
                    await interaction.editReply("You cannot specify a reason when the type is not custom.");
                    return;
                }

                let result;
                try {
                    result = await this.client.orionAPI.createCreditTransaction(user.id, {
                        type,
                        amount,
                        reason: reason || null,
                    });
                } catch (err) {
                    if (err instanceof OrionAPIError) {
                        await interaction.editReply(`Failed to add credits: ${err.message} (status: ${err.status})`);
                        return;
                    }

                    throw err;
                }

                await interaction.editReply(
                    `Credits added, the user has now ${result.user_credits} credits. Transaction ID: ${result.transaction_id}`,
                );
                return;
            }
        }

        await interaction.editReply("Command not implemented.");
    }
}
