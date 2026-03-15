import { AdService } from "@/services/ad";
import { BoostService } from "@/services/boost";
import { ChatbotService } from "@/services/chatbot";
import { StatusService } from "@/services/status";
import type { OrionBot } from "@/structures/index";
import type { Service } from "./service";

export default class ServiceManager {
    public ready: boolean;
    public readonly registered: Service[];

    public readonly ad: AdService;
    public readonly chatbot: ChatbotService;
    public readonly boosts: BoostService;
    public readonly status: StatusService;

    public constructor(public readonly client: OrionBot) {
        this.ready = false;
        this.registered = [];

        this.ad = this._register(new AdService(client));
        this.chatbot = this._register(new ChatbotService(client));
        this.boosts = this._register(new BoostService(client));
        this.status = this._register(new StatusService(client));
    }

    public onReady(): void {
        if (this.ready) throw new Error("ServiceManager already ready");
        this.ready = true;

        for (const service of this.registered) {
            service.onReady?.();
        }
    }

    private _register<T extends Service>(service: T): T {
        this.registered.push(service);
        return service;
    }
}
