import type { RateLimitData } from "discord.js";
import { Event, EventType, type EventData } from "@/structures/index";

export default class extends Event {
    public static override readonly data: EventData = {
        name: "rateLimited",
        type: EventType.Rest,
    };

    public handle(rateLimitData: RateLimitData): void {
        const formatted = Object.entries(rateLimitData)
            .map(([k, v]) => `${k}: ${v}`)
            .join("\n");
        this.client.logger.warn("RateLimited:");
        this.client.logger.warn(formatted);
    }
}
