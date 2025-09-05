import { Event, EventType, type EventData } from "@/structures/index";

export default class extends Event {
    public static override readonly data: EventData = {
        name: "uncaughtException",
        type: EventType.Process,
    };

    public handle(err: Error, origin: string): void {
        this.client.remoteLogger.sendError(`UncaughtException:${origin}`, err);
    }
}
