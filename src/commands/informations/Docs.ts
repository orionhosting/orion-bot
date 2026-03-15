import {
    ApplicationCommandOptionType,
    ApplicationIntegrationType,
    InteractionContextType,
    MessageFlags,
} from "discord.js";
import {
    AutocompleteHandlerContext,
    Command,
    CommandCategory,
    type CommandData,
    type CommandHandlerContext,
} from "@/structures/index";

export default class extends Command {
    public static override readonly data: CommandData = {
        name: "docs",
        category: CommandCategory.Informations,
        integrationTypes: [ApplicationIntegrationType.GuildInstall, ApplicationIntegrationType.UserInstall],
        contexts: [InteractionContextType.Guild, InteractionContextType.BotDM, InteractionContextType.PrivateChannel],
        options: [
            {
                name: "page",
                type: ApplicationCommandOptionType.String,
                autocomplete: true,
            },
        ],
    };

    public override async handleCommand({ lang, interaction }: CommandHandlerContext) {
        let inputPage = interaction.options.getString("page") || `${this.config.docsURL}/${lang.meta.docsLocale}`;

        const sitemap = await this.client.cache.getDocumentationSitemap();
        const page = sitemap.find(p => p.url === inputPage);
        if (!page) {
            await interaction.reply({
                content: `${this.emojis.warn} ${lang.t("err_invalid_page")}`,
                flags: MessageFlags.Ephemeral,
            });
            return;
        }

        await interaction.reply({
            content: `**${this.emojis.property} [${page.name}](${page.url}) **` + `\n${page.description}`,
            allowedMentions: { parse: [] },
        });
    }

    public override async handleAutocomplete({ interaction }: AutocompleteHandlerContext): Promise<void> {
        const inputPage = interaction.options.getFocused().toLowerCase();

        const sitemap = await this.client.cache.getDocumentationSitemap();
        const options = sitemap.filter(u => u.name.toLowerCase().includes(inputPage));

        await interaction.respond(
            options.slice(0, 25).map(o => ({
                name: `${o.lang} - ${o.name}`,
                value: o.url,
            })),
        );
    }
}
