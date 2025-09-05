import { Command, CommandCategory, type CommandData, type SlashHandlerContext } from "@/structures/index";

export default class extends Command {
    public static override readonly data: CommandData = {
        name: "config",
        category: CommandCategory.Private,
        ownerOnly: true,
    };

    public override async handleSlash({ interaction }: SlashHandlerContext) {
        await interaction.reply({ content: "Command not implemented.", ephemeral: true });
    }
}
