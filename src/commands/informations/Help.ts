import { StringSelectMenuBuilder, ActionRowBuilder, type APIEmbed } from "discord.js";
import {
    Command,
    CommandCategory,
    type CommandData,
    type SlashHandlerContext,
    type ComponentHandlerContext,
    type LocaleContext,
} from "@/structures/index";

export default class extends Command {
    public static override readonly data: CommandData = {
        name: "help",
        category: CommandCategory.Informations,
    };

    public override async handleSlash({ lang, interaction }: SlashHandlerContext) {
        await interaction.reply({
            embeds: [this.createHomeEmbed(lang)],
            components: this.createComponents(lang, interaction.user.id),
        });
    }

    public override async handleComponent({ lang, interaction }: ComponentHandlerContext) {
        if (!interaction.isStringSelectMenu()) return;

        const userId = interaction.customId.split("-").at(-1);
        if (interaction.user.id !== userId) {
            await interaction.reply({
                content: `${this.emojis.warn} ${lang.common("errors.err_menu_owner")}`,
                ephemeral: true,
            });
            return;
        }

        const options = this._getOptions(lang);
        const selectedOption = options.find(o => o.value === interaction.values[0]);

        if (!selectedOption) return;

        if (selectedOption.value === "home") {
            await interaction.update({
                embeds: [this.createHomeEmbed(lang)],
                components: this.createComponents(lang, interaction.user.id),
            });
            return;
        }

        const commands = this.client.commands
            .filter(c => c.category === selectedOption.category)
            .map(
                c =>
                    `\`/${c.name}\`: ${lang.ngt(`commands:${c.name}.description`) || lang.gt("common:commands.no_description")}`,
            );

        if (!commands.length) {
            await interaction.reply({ content: `${this.emojis.warn} ${lang.t("err_no_commands")}`, ephemeral: true });
            return;
        }

        const embed: APIEmbed = {
            color: this.colors.primary.int,
            title: `${selectedOption.emoji} ${lang.t("help_menu")} - ${selectedOption.label}`,
            description: commands.join("\n"),
        };

        await interaction.update({ embeds: [embed] });
    }

    private createHomeEmbed(lang: LocaleContext) {
        return {
            color: this.colors.primary.int,
            title: lang.t("help_menu"),
            thumbnail: {
                url: this.client.user.displayAvatarURL(),
            },
            description:
                `> ${lang.t("home.desc")}` +
                `\n> ${lang.t("home.tickets", { channel: `<#${this.config.ticketChannelId}>` })}`,
            fields: [
                {
                    name: lang.t("home.links.title"),
                    value:
                        `> [${lang.t("home.links.website")}](${this.config.domainURL})` +
                        `\n> [${lang.t("home.links.panel")}](${this.config.panelURL})`,
                },
            ],
        } satisfies APIEmbed;
    }

    private createComponents(lang: LocaleContext, userId: string) {
        const row = new ActionRowBuilder<StringSelectMenuBuilder>().setComponents(
            new StringSelectMenuBuilder().setCustomId(`help-menu-${userId}`).setOptions(
                this._getOptions(lang).map(o => ({
                    label: o.label,
                    emoji: o.emoji,
                    value: o.value,
                })),
            ),
        );

        return [row];
    }

    public _getOptions(lang: LocaleContext) {
        return [
            {
                label: lang.t("selector.home.label"),
                emoji: "🏠",
                value: "home",
            },
            {
                label: lang.gt("common:commands.categories.informations"),
                emoji: "ℹ️",
                value: "informations",
                category: CommandCategory.Informations,
            },
        ];
    }
}
