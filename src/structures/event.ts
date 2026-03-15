import { config, colors, emojis } from "@/config/index";
import type { OrionBot } from "./bot";

export interface EventFile {
    default: {
        readonly data: EventData;
        new (client: OrionBot, data: EventData, filepath: string): Event;
    };
}

export enum EventType {
    Process,
    Client,
    Rest,
}

export interface EventData {
    name: string;
    type: EventType;
}

export abstract class Event implements EventData {
    public static readonly data: EventData;

    public readonly filepath: string;

    public readonly name: string;
    public readonly type: EventType;

    public readonly config: typeof config;
    public readonly colors: typeof colors;
    public readonly emojis: typeof emojis;

    public constructor(
        public readonly client: OrionBot,
        data: EventData,
        filepath: string,
    ) {
        this.filepath = filepath;

        this.name = data.name;
        this.type = data.type;

        this.config = config;
        this.colors = colors;
        this.emojis = emojis;
    }
}
