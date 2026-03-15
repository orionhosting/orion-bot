import assert from "node:assert";
import { Prisma, PrismaPromise, UserBoostState } from "@prisma-generated/index";
import { Time } from "@sodiumlabs/duration";
import type { Collection, GuildMember } from "discord.js";
import { config } from "@/config/index";
import { CreditTransactionType } from "@/database/enums";
import { prisma } from "@/database/prisma";
import type { OrionBot } from "@/structures/index";
import { Service } from "@/structures/services/service";

export class BoostService extends Service {
    public override readonly data = {
        name: "boost",
    };

    public readonly updateInterval = Time.Minute * 20;

    public constructor(client: OrionBot) {
        super(client);
    }

    public override onReady() {
        if (process.env.NODE_ENV !== "production") return;

        setInterval(async () => {
            try {
                await this.tick();
            } catch (err) {
                this.client.monitor.captureException(err, "Boost update");
            }
        }, this.updateInterval);
    }

    /**
     *
     * @throws If `state.boosting_since` is null
     */
    public getNextRewardTimestamp(state: Pick<UserBoostState, "boosting_since" | "last_reward_at">) {
        assert(state.boosting_since !== null);
        return (state.boosting_since > state.last_reward_at ? state.boosting_since : state.last_reward_at) + Time.Week;
    }

    /**
     * Boosts check interval.
     */
    private async tick() {
        const guild = this.client.guilds.cache.get(config.supportGuildId);
        assert(guild, "support guild not found");

        const members = await guild.members.fetch();
        const states = await prisma.userBoostState.findMany();

        const operations: PrismaPromise<UserBoostState | Prisma.BatchPayload>[] = [];
        /**
         * Discord IDs of the users
         */
        const pendingRewards: string[] = [];

        this.updateStates(states, members, operations, pendingRewards);
        this.addNewBoosters(states, members, operations, pendingRewards);

        // Save transactions

        let amount = 10;
        try {
            await this.client.orionAPI.createMassCreditTransaction(
                pendingRewards.map(id => ({
                    discord_id: id,
                    type: CreditTransactionType.DiscordBoost,
                    amount,
                    reason: null,
                })),
            );
        } catch (err) {
            this.client.logger.info(err, "Could not add boosts rewards");
            return;
        }

        // Save states in db

        await prisma.$transaction(operations);

        this.client.logger.info(`Ran ${operations.length} operations during boost states update`);

        // Alert on Discord

        await this.client.remoteLogger.sendInfo(
            "log",
            `[Boost rewards] ${pendingRewards.length} users just received ${amount} credits each. (${pendingRewards.map(id => `<@${id}>`).join(", ")})`,
        );
    }

    /**
     * Update the existing states (members that boosted at least once before).
     *
     * @param operations will be mutated.
     * @param pendingRewards will be mutated.
     */
    private updateStates(
        states: UserBoostState[],
        members: Collection<string, GuildMember>,
        operations: PrismaPromise<UserBoostState | Prisma.BatchPayload>[],
        pendingRewards: string[],
    ) {
        const now = Date.now();

        for (const state of states) {
            const member = members.get(state.user_id);
            let hasChanged = false;

            if (state.is_boosting) {
                if (!member || !member.premiumSinceTimestamp) {
                    // Is not boosting anymore
                    hasChanged = true;
                }
            } else {
                if (member && member.premiumSinceTimestamp) {
                    // Is now boosting
                    hasChanged = true;
                }
            }

            // Update the state if needed
            if (hasChanged) {
                if (!state.is_boosting) pendingRewards.push(state.user_id);

                operations.push(
                    prisma.userBoostState.update({
                        where: { user_id: state.user_id },
                        data: {
                            is_boosting: !state.is_boosting,
                            boosting_since: state.is_boosting ? null : now,
                            updated_at: now,
                        },
                    }),
                );
            } else {
                // Or update the timestamp only
                operations.push(
                    prisma.userBoostState.update({
                        where: { user_id: state.user_id },
                        data: {
                            updated_at: now,
                        },
                    }),
                );
            }

            if (!hasChanged) {
                let rewarded = false;
                if (state.boosting_since !== null && Date.now() > this.getNextRewardTimestamp(state)) {
                    // It's reward time
                    pendingRewards.push(state.user_id);
                    rewarded = true;
                }

                operations.push(
                    prisma.userBoostState.update({
                        where: { user_id: state.user_id },
                        data: {
                            last_reward_at: rewarded ? now : undefined,
                            updated_at: now,
                        },
                    }),
                );
            }
        }
    }

    /**
     * Create stats for new boosters.
     *
     * @param operations will be mutated.
     * @param pendingRewards will be mutated.
     */
    private addNewBoosters(
        states: UserBoostState[],
        members: Collection<string, GuildMember>,
        operations: PrismaPromise<UserBoostState | Prisma.BatchPayload>[],
        pendingRewards: string[],
    ) {
        const now = Date.now();
        const newBoosters = [];

        for (const member of members.values()) {
            if (!member.premiumSinceTimestamp) continue;
            if (states.find(s => s.user_id === member.id)) continue;
            newBoosters.push(member.id);
            pendingRewards.push(member.id);
        }

        if (newBoosters.length) {
            operations.push(
                prisma.userBoostState.createMany({
                    data: newBoosters.map(b => ({
                        user_id: b,
                        is_boosting: true,
                        boosting_since: now,
                        updated_at: now,
                        last_reward_at: now,
                    })),
                }),
            );
        }
    }
}
