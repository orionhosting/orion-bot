import { Time } from "@voctal/duration";
import { CreditTransactionType } from "@/database/enums";
import type { OrionBot } from "@/structures/index";
import { Service } from "@/structures/services/service";
import { prisma } from "@/database/prisma";

export class AdService extends Service {
  public override readonly data = {
    name: "ad",
  };

  public readonly updateInterval = Time.Day;

  public constructor(client: OrionBot) {
    super(client);
  }

  public override onReady() {
    if (process.env.NODE_ENV !== "production") return;

    this.tick().catch((err) =>
      this.client.monitor.captureException(err, "Ad update init"),
    );

    setInterval(async () => {
      try {
        await this.tick();
      } catch (err) {
        this.client.monitor.captureException(err, "Ad update");
      }
    }, this.updateInterval);
  }

  private async tick() {
    const settingsList = await prisma.guildSettings.findMany({
      where: {
        ad_enabled: true,
        ad_channel_id: { not: null },
        ad_message_id: { not: null },
      },
    });

    const now = Math.floor(Date.now() / 1000);

    for (const settings of settingsList) {
      let isValid = false;
      const guild = this.client.guilds.cache.get(settings.id);

      try {
        if (guild) {
          const channel = await guild.channels
            .fetch(settings.ad_channel_id!)
            .catch(() => null);

          if (channel && channel.isTextBased()) {
            const isNsfw = "nsfw" in channel ? channel.nsfw : false;

            if (!isNsfw) {
              const everyoneRole = guild.roles.everyone;
              const perms = channel.permissionsFor(everyoneRole);

              if (perms && perms.has("ViewChannel")) {
                const messages = await channel.messages
                  .fetch({ limit: 1 })
                  .catch(() => null);
                const lastMessage = messages?.first();

                if (
                  lastMessage &&
                  lastMessage.author.id === this.client.user?.id
                ) {
                  isValid = true;
                }
              }
            }
          }
        }
      } catch (err) {
        this.client.logger.error(
          `Error checking ad for ${settings.id}: ${err}`,
        );
      }

      const currentState = await prisma.guildAdState.findUnique({
        where: { guild_id: settings.id },
      });

      const validSince = isValid
        ? currentState?.is_valid && currentState?.valid_since
          ? currentState.valid_since
          : now
        : null;

      let lastRewardAt = currentState?.last_reward_at ?? null;

      if (isValid && guild) {
        const oneWeek = 7 * 24 * 60 * 60;
        const timeSinceValid = now - (validSince ?? now);
        const timeSinceLastReward = lastRewardAt ? now - lastRewardAt : null;

        if (timeSinceValid >= oneWeek) {
          if (timeSinceLastReward === null || timeSinceLastReward >= oneWeek) {
            const memberCount = guild.memberCount;
            let amount = 0;
            if (memberCount >= 1000) amount = 40;
            else if (memberCount >= 500) amount = 35;
            else if (memberCount >= 250) amount = 30;
            else if (memberCount >= 20) amount = 20;

            if (amount > 0) {
              try {
                await this.client.orionAPI.createCreditTransaction(
                  guild.ownerId,
                  {
                    type: CreditTransactionType.DiscordSponsoredAd,
                    amount,
                    reason: `Ad reward`,
                  },
                );
                lastRewardAt = now;
              } catch (err) {
                this.client.logger.error(
                  `Error rewarding guild ${guild.id}: ${err}`,
                );
              }
            }
          }
        }
      }

      await prisma.guildAdState.upsert({
        where: { guild_id: settings.id },
        update: {
          is_valid: isValid,
          updated_at: now,
          valid_since: validSince,
          last_reward_at: lastRewardAt,
        },
        create: {
          guild_id: settings.id,
          is_valid: isValid,
          updated_at: now,
          valid_since: validSince,
          last_reward_at: lastRewardAt,
        },
      });
    }
  }
}
