import {
  ActionRowBuilder,
  ApplicationCommandOptionType,
  ApplicationIntegrationType,
  ButtonBuilder,
  ButtonStyle,
  ContainerBuilder,
  InteractionContextType,
  MessageActionRowComponent,
  MessageActionRowComponentBuilder,
  MessageFlags,
  SectionBuilder,
  SeparatorBuilder,
  SeparatorSpacingSize,
  TextDisplayBuilder,
  ThumbnailBuilder,
} from "discord.js";
import {
  Command,
  CommandCategory,
  ComponentHandlerContext,
  type CommandData,
  type CommandHandlerContext,
} from "@/structures/index";
import { prisma } from "@/database/prisma";

export default class extends Command {
  public static override readonly data: CommandData = {
    name: "ad",
    category: CommandCategory.Informations,
    integrationTypes: [ApplicationIntegrationType.GuildInstall],
    contexts: [
      InteractionContextType.Guild,
      InteractionContextType.BotDM,
      InteractionContextType.PrivateChannel,
    ],
    options: [
      {
        name: "config",
        type: ApplicationCommandOptionType.Subcommand,
        description: "Configure our ad",
        options: [
          {
            name: "channel",
            type: ApplicationCommandOptionType.Channel,
            description: "The channel to send the ad in",
            required: true,
          },
        ],
      },
      {
        name: "status",
        type: ApplicationCommandOptionType.Subcommand,
        description: "Check ad status",
      },
    ],
  };

  public override async handleCommand({
    lang,
    interaction,
  }: CommandHandlerContext) {
    const subcommand = interaction.options.getSubcommand();

    if (subcommand === "config") {
      const channel = interaction.options.getChannel("channel", true);

      const container = new ContainerBuilder()
        .setAccentColor(this.colors.primary.int)
        .addTextDisplayComponents(
          new TextDisplayBuilder().setContent(
            `## ${this.emojis.logo} Orion - ${lang.t("config.title")}`,
          ),
        )
        .addTextDisplayComponents(
          new TextDisplayBuilder().setContent(
            lang.t("config.message", { channel: channel.toString() }),
          ),
        )
        .addActionRowComponents(
          new ActionRowBuilder<MessageActionRowComponentBuilder>().addComponents(
            new ButtonBuilder()
              .setCustomId("ad-send_ad-" + channel.id)
              .setLabel(lang.t("config.buttons.send"))
              .setStyle(ButtonStyle.Primary),
          ),
        );

      await interaction.reply({
        components: [container],
        flags: [MessageFlags.IsComponentsV2, MessageFlags.Ephemeral],
        allowedMentions: { parse: [] },
      });
    } else if (subcommand === "status") {
      if (!interaction.guildId) return;

      const settings = await prisma.guildSettings.findUnique({
        where: { id: interaction.guildId }
      });
      const state = await prisma.guildAdState.findUnique({
        where: { guild_id: interaction.guildId }
      });

      const isEnabled = settings?.ad_enabled ? lang.t("status.enabled") : lang.t("status.disabled");
      const channelStr = settings?.ad_channel_id ? `<#${settings.ad_channel_id}>` : lang.t("status.none");
      const isValid = state?.is_valid ? lang.t("status.valid") : lang.t("status.invalid");
      const validSince = state?.valid_since ? `<t:${state.valid_since}:R>` : lang.t("status.na");
      const lastReward = state?.last_reward_at ? `<t:${state.last_reward_at}:R>` : lang.t("status.na");

      const container = new ContainerBuilder()
        .setAccentColor(this.colors.primary.int)
        .addTextDisplayComponents(
          new TextDisplayBuilder().setContent(`## ${this.emojis.logo} Orion - ${lang.t("status.title")}`),
        )
        .addTextDisplayComponents(
          new TextDisplayBuilder().setContent(
            `${lang.t("status.global_status", { status: isEnabled })}\n${lang.t("status.channel", { channel: channelStr })}\n\n${lang.t("status.current_state", { state: isValid })}\n${lang.t("status.valid_since", { date: validSince })}\n${lang.t("status.last_reward", { date: lastReward })}`
          )
        );

      await interaction.reply({
        components: [container],
        flags: [MessageFlags.IsComponentsV2, MessageFlags.Ephemeral],
        allowedMentions: { parse: [] },
      });
    }
  }

  public override async handleComponent({
    interaction,
    lang,
  }: ComponentHandlerContext) {
    if (
      interaction.isMessageComponent() &&
      interaction.customId.startsWith("ad-send_ad-")
    ) {
      const pub = lang.t("config.pub");
      const channelId = interaction.customId.split("ad-send_ad-")[1];
      if (!channelId) return;

      const channel = await this.client.channels
        .fetch(channelId)
        .catch(() => null);
      if (!channel || !channel.isTextBased() || !("send" in channel)) {
        await interaction.reply({
          content: this.emojis.warn + " Invalid channel.",
          allowedMentions: { parse: [] },
          flags: [MessageFlags.Ephemeral],
        });
        return;
      }

      const message = await channel.send({
        content: pub,
        allowedMentions: { parse: [] },
      });

      if (!interaction.guildId) return;

      await prisma.guildSettings.upsert({
        where: { id: interaction.guildId },
        update: {
          ad_enabled: true,
          ad_channel_id: channel.id,
          ad_message_id: message.id,
        },
        create: {
          id: interaction.guildId,
          ad_enabled: true,
          ad_channel_id: channel.id,
          ad_message_id: message.id,
        },
      });

      await interaction.reply({
        content:
          this.emojis.success +
          " " +
          lang.t("config.success", {
            channel: channel ? channel.toString() : "unknown channel",
          }),
        allowedMentions: { parse: [] },
        flags: [MessageFlags.Ephemeral],
      });
    }
  }
}
