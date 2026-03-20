import {
    ActionRowBuilder,
    ApplicationCommandOptionType,
    ButtonBuilder,
    ButtonStyle,
    ContainerBuilder,
    MediaGalleryBuilder,
    MediaGalleryItemBuilder,
    MessageFlags,
    SectionBuilder,
    SeparatorBuilder,
    SeparatorSpacingSize,
    TextDisplayBuilder,
} from "discord.js";
import { config } from "@/config";
import { Command, CommandCategory, type CommandData, type CommandHandlerContext } from "@/structures/index";

export default class extends Command {
    public static override readonly data: CommandData = {
        name: "panels",
        category: CommandCategory.Private,
        description: "Send custom panels",
        guilds: [config.supportGuildId],
        ownerOnly: true,
        options: [
            {
                name: "name",
                type: ApplicationCommandOptionType.String,
                description: "The panel name",
                required: true,
            },
        ],
    };

    public override async handleCommand({ interaction }: CommandHandlerContext) {
        if (!interaction.inCachedGuild()) return;
        if (!interaction.channel?.isSendable()) return;

        await interaction.deferReply({ flags: MessageFlags.Ephemeral });

        let components;
        switch (interaction.options.getString("name", true)) {
            case "links":
                components = this.getLinksComponents();
                break;
            case "rules":
                components = this.getRulesComponents();
                break;
            default:
                await interaction.editReply("Invalid name!");
                return;
        }

        await interaction.channel.send({
            components,
            flags: MessageFlags.IsComponentsV2,
            allowedMentions: {
                parse: [],
            },
        });

        await interaction.editReply("Sent!");
    }

    private getLinksComponents() {
        const container = new ContainerBuilder()
            .setAccentColor(this.colors.primary.int)
            .addTextDisplayComponents(new TextDisplayBuilder().setContent(`## ${this.emojis.logo} Orion - Links`))
            .addMediaGalleryComponents(
                new MediaGalleryBuilder().addItems(new MediaGalleryItemBuilder().setURL(this.config.bannerURL)),
            )
            .addSectionComponents(
                new SectionBuilder()
                    .addTextDisplayComponents(
                        new TextDisplayBuilder().setContent(
                            "> Explore our hosting services or login in the dashboard.",
                        ),
                    )
                    .setButtonAccessory(
                        new ButtonBuilder()
                            .setStyle(ButtonStyle.Link)
                            .setURL(this.config.domainURL)
                            .setEmoji({ id: this.emojis.logoId })
                            .setLabel("Website / Dashboard"),
                    ),
            )
            .addSeparatorComponents(new SeparatorBuilder())
            .addSectionComponents(
                new SectionBuilder()
                    .addTextDisplayComponents(
                        new TextDisplayBuilder().setContent("> Use the panel to manage your servers."),
                    )
                    .setButtonAccessory(
                        new ButtonBuilder()
                            .setStyle(ButtonStyle.Link)
                            .setURL(this.config.panelURL)
                            .setEmoji({ name: "🔗" })
                            .setLabel("Panel"),
                    ),
            )
            .addSeparatorComponents(new SeparatorBuilder())
            .addSectionComponents(
                new SectionBuilder()
                    .addTextDisplayComponents(
                        new TextDisplayBuilder().setContent(
                            "> Read the documentation to learn how to use our hosting.",
                        ),
                    )
                    .setButtonAccessory(
                        new ButtonBuilder()
                            .setStyle(ButtonStyle.Link)
                            .setURL(this.config.docsURL)
                            .setEmoji({ name: "📚" })
                            .setLabel("Documentation"),
                    ),
            )
            .addSeparatorComponents(new SeparatorBuilder())
            .addSectionComponents(
                new SectionBuilder()
                    .addTextDisplayComponents(
                        new TextDisplayBuilder().setContent("> Get the status of our services in real-time."),
                    )
                    .setButtonAccessory(
                        new ButtonBuilder()
                            .setStyle(ButtonStyle.Link)
                            .setURL(this.config.statusURL)
                            .setEmoji({ id: this.emojis.propertyId })
                            .setLabel("Status"),
                    ),
            )
            .addSeparatorComponents(new SeparatorBuilder())
            .addSectionComponents(
                new SectionBuilder()
                    .addTextDisplayComponents(new TextDisplayBuilder().setContent("> Need help? Open a ticket!"))
                    .setButtonAccessory(
                        new ButtonBuilder()
                            .setStyle(ButtonStyle.Link)
                            .setURL(this.config.ticketsPanelURL)
                            .setEmoji({ id: this.emojis.discordId })
                            .setLabel("Open a ticket"),
                    ),
            );

        const legal = new ContainerBuilder()
            .setAccentColor(this.colors.primary.int)
            .addTextDisplayComponents(new TextDisplayBuilder().setContent(`## ${this.emojis.logo} Legal Links`))
            .addTextDisplayComponents(
                new TextDisplayBuilder().setContent(
                    `> Terms of Service: ${this.config.domainURL}/terms` +
                        `\n> Privacy Policy: ${this.config.domainURL}/privacy`,
                ),
            );

        return [container, legal];
    }

    private getRulesComponents() {
        const container = new ContainerBuilder()
            .setAccentColor(this.colors.primary.int)
            .addTextDisplayComponents(new TextDisplayBuilder().setContent(`## ${this.emojis.logo} Orion - Rules`))
            .addMediaGalleryComponents(
                new MediaGalleryBuilder().addItems(new MediaGalleryItemBuilder().setURL(this.config.bannerURL)),
            )
            .addTextDisplayComponents(
                new TextDisplayBuilder().setContent(
                    "## Civilized behavior" +
                        "\nTreat other members with respect, even if you disagree with them." +
                        "\nHate speech, harassment and discriminatory language are strictly forbidden." +
                        "\nPlease respect the decisions of moderators and administrators.",
                ),
            )
            .addSeparatorComponents(new SeparatorBuilder().setSpacing(SeparatorSpacingSize.Large))
            .addTextDisplayComponents(
                new TextDisplayBuilder().setContent(
                    "## Privacy" +
                        "\nPrivate messages (PMs) may not be used for advertising or spamming." +
                        "\nInvitations to other servers or the addition of bots are strictly forbidden." +
                        "\nMembers' personal information must not be shared without their consent.",
                ),
            )
            .addSeparatorComponents(new SeparatorBuilder().setSpacing(SeparatorSpacingSize.Large))
            .addTextDisplayComponents(
                new TextDisplayBuilder().setContent(
                    "## Security" +
                        "\nPasswords and other sensitive information must not be shared." +
                        "\nReport any suspicious behavior or fraudulent activity to the moderators.",
                ),
            )
            .addSeparatorComponents(new SeparatorBuilder().setSpacing(SeparatorSpacingSize.Large))
            .addTextDisplayComponents(
                new TextDisplayBuilder().setContent(
                    "## Other" +
                        "\nLinks to malicious websites or explicit content are prohibited." +
                        "\nExcessive use of capital letters, spam or animated emotes is prohibited." +
                        "\nMembers may not impersonate others.",
                ),
            )
            .addSeparatorComponents(new SeparatorBuilder().setSpacing(SeparatorSpacingSize.Large))
            .addTextDisplayComponents(
                new TextDisplayBuilder().setContent(
                    "## Civilized behavior" +
                        "\nTreat other members with respect, even if you disagree with them." +
                        "\nHate speech, harassment and discriminatory language are strictly forbidden." +
                        "\nPersonal conflicts should be resolved privately or with the help of a moderator." +
                        "\nPlease respect the decisions of moderators and administrators.",
                ),
            );

        const row = new ActionRowBuilder<ButtonBuilder>().setComponents(
            new ButtonBuilder()
                .setStyle(ButtonStyle.Link)
                .setURL(this.config.domainURL)
                .setEmoji({ id: this.emojis.logoId })
                .setLabel("Website"),
            new ButtonBuilder()
                .setStyle(ButtonStyle.Link)
                .setURL(`${this.config.domainURL}/terms`)
                .setEmoji({ id: this.emojis.logoId })
                .setLabel("ToS"),
            new ButtonBuilder()
                .setStyle(ButtonStyle.Link)
                .setURL(`${this.config.domainURL}/privacy`)
                .setEmoji({ id: this.emojis.logoId })
                .setLabel("Privacy Policy"),
        );

        return [container, row];
    }
}
