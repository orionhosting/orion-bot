import { Time } from "@sodiumlabs/duration";
import type { OrionBot } from "@/structures/index";
import { Service } from "@/structures/services/service";

export class AdService extends Service {
    public override readonly data = {
        name: "ad",
    };

    public readonly updateInterval = Time.Hour;

    public constructor(client: OrionBot) {
        super(client);
    }

    public override onReady() {
        if (process.env.NODE_ENV !== "production") return;

        setInterval(async () => {
            try {
                // await this.tick();
            } catch (err) {
                this.client.monitor.captureException(err, "Ad update");
            }
        }, this.updateInterval);
    }
}
