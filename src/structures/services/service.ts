import { Locale } from "discord.js";
import { config, colors, emojis } from "@/config/index";
import type { LocaleContext, OrionBot } from "@/structures/index";

export interface ServiceData {
    name: string;
}

/**
 * Represents a service.
 */
export abstract class Service {
    public abstract readonly data: ServiceData;

    public readonly config: typeof config;
    public readonly colors: typeof colors;
    public readonly emojis: typeof emojis;

    private _disabledAt: number | null;
    private readonly _client: OrionBot;

    public constructor(client: OrionBot) {
        this._client = client;
        this.config = config;
        this.colors = colors;
        this.emojis = emojis;
        this._disabledAt = null;
    }

    public get disabledAt() {
        return this._disabledAt;
    }

    public get enabled() {
        return this._disabledAt === null;
    }

    public set enabled(v: boolean) {
        this._disabledAt = v ? Date.now() : null;
    }

    public get client(): OrionBot<true> {
        if (!this._client.isReady()) throw new Error("client is not ready");
        return this._client;
    }

    public onReady?(): void | Promise<void>;

    protected getLang(locale: Locale): LocaleContext<"services", string> {
        return this.client.getLocale(locale, "services", this.data.name);
    }
}
