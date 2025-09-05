import { Events } from "discord.js";
import { Event, EventType, type EventData } from "@/structures/index";

export default class extends Event {
    public static override readonly data: EventData = {
        name: Events.ClientReady,
        type: EventType.Client,
    };

    public async handle(): Promise<void> {
        const guild = this.client.guilds.cache.get(this.config.supportGuildId);
        if (!guild) {
            console.error("Cannot find the support guild");
            process.exit(1);
        }

        console.log("Client ready");
    }
}
