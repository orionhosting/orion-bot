import { ModalBuilder, TextInputBuilder, TextInputStyle, ActionRowBuilder, type APIEmbed } from "discord.js";
import {
    Command,
    CommandCategory,
    type ComponentHandlerContext,
    type CommandData,
    type SlashHandlerContext,
} from "@/structures/index";

export default class extends Command {
    public static override readonly data: CommandData = {
        name: "eval",
        category: CommandCategory.Private,
        ownerOnly: true,
    };

    public override async handleSlash({ interaction }: SlashHandlerContext) {
        const modal = new ModalBuilder()
            .setCustomId("eval-modal")
            .setTitle("Eval")
            .setComponents(
                new ActionRowBuilder<TextInputBuilder>().setComponents(
                    new TextInputBuilder()
                        .setCustomId("code")
                        .setLabel("Code")
                        .setRequired(true)
                        .setStyle(TextInputStyle.Paragraph),
                ),
            );

        await interaction.showModal(modal);
    }

    public override async handleComponent({ interaction }: ComponentHandlerContext) {
        if (!interaction.isModalSubmit()) return;

        const code = interaction.fields.getTextInputValue("code");
        await interaction.deferReply();

        // oxlint-disable-next-line no-eval
        const result: unknown = await eval(`(async()=>{${code}})()`);
        const embed: APIEmbed = {
            color: this.colors.primary.int,
            fields: [
                {
                    name: "Code",
                    value: `\`\`\`\n${`${code}`.slice(0, 1024)}\n\`\`\``,
                },
                {
                    name: `Résultat (${typeof result})`,
                    value: `\`\`\`\n${`${JSON.stringify(result, null, 2)}`.slice(0, 1024)}\n\`\`\``,
                },
            ],
        };

        await interaction.editReply({ embeds: [embed] });
    }
}
