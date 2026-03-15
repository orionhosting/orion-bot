import { Events } from "discord.js";
import { Event, EventType, type EventData } from "@/structures/index";

export default class extends Event {
    public static override readonly data: EventData = {
        name: Events.ClientReady,
        type: EventType.Client,
    };

    public async handle(): Promise<void> {
        // This will crash the process if the guild is not found
        this.client.getSupportGuild();

        this.client.logger.info("Client ready");

        this.client.services.onReady();
    }
}
