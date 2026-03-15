import { inspect } from "node:util";
import { ModalBuilder, TextInputBuilder, TextInputStyle, type APIEmbed, LabelBuilder } from "discord.js";
import { config } from "@/config";
import {
    Command,
    CommandCategory,
    type ComponentHandlerContext,
    type CommandData,
    type CommandHandlerContext,
} from "@/structures/index";

export default class extends Command {
    public static override readonly data: CommandData = {
        name: "eval",
        category: CommandCategory.Private,
        description: "Evaluate a script",
        guilds: [config.supportGuildId],
        ownerOnly: true,
    };

    public override async handleCommand({ interaction }: CommandHandlerContext) {
        const modal = new ModalBuilder()
            .setCustomId("eval-modal")
            .setTitle("Eval")
            .addLabelComponents(
                new LabelBuilder()
                    .setLabel("Code")
                    .setDescription("The code to run")
                    .setTextInputComponent(
                        new TextInputBuilder().setCustomId("code").setRequired(true).setStyle(TextInputStyle.Paragraph),
                    ),
            );

        await interaction.showModal(modal);
    }

    public override async handleComponent({ interaction }: ComponentHandlerContext) {
        if (!interaction.isModalSubmit()) return;

        const code = interaction.fields.getTextInputValue("code");
        await interaction.deferReply();

        const stringify = (result: unknown) => `\`\`\`\n${inspect(result)}\n\`\`\``;
        let result;

        try {
            // oxlint-disable-next-line no-eval
            const evaluated = await eval(`(async()=>{${code}})()`);
            result = stringify(evaluated);
        } catch (err) {
            result = "Error:" + `\n${stringify(err instanceof Error ? err.stack : err)}`;
        }

        const embed: APIEmbed = {
            color: this.colors.primary.int,
            fields: [
                {
                    name: "Code",
                    value: `\`\`\`\n${`${code}`.slice(0, 1024)}\n\`\`\``,
                },
                {
                    name: `Result (${typeof result})`,
                    value: result,
                },
            ],
        };

        await interaction.editReply({ embeds: [embed] });
    }
}
