import { ClientEvents, Events } from "discord.js";
import { Event, EventType, type EventData } from "@/structures/index";

export default class extends Event {
    public static override readonly data: EventData = {
        name: Events.MessageCreate,
        type: EventType.Client,
    };

    public async handle(...[message]: ClientEvents["messageCreate"]): Promise<void> {
        if (!message.inGuild()) return;

        this.client.services.chatbot
            .onMessage(message)
            .catch(err => this.client.monitor.captureException(err, "Chatbot onMessage"));
    }
}
