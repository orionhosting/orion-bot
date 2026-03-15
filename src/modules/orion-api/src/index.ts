import z from "zod";
import { CreditTransactionType } from "@/database/enums";
import { REST } from "./rest";

export interface OrionAPIOptions {
    key: string;
    url: string;
}

/**
 * Tiny wrapper around Orion API.
 */
export class OrionAPIClient {
    public readonly key: string;
    public readonly url: string;
    public readonly rest: REST;

    public constructor(options: OrionAPIOptions) {
        this.key = options.key;
        this.url = options.url;
        this.rest = new REST(this);
    }

    public async getStatus() {
        const schema = z.object({
            user_count: z.int(),
            server_count: z.int(),
            suspended_server_count: z.int(),
            node_count: z.int(),
            nodes: z.array(
                z.object({
                    id: z.int(),
                    name: z.string(),
                    maintenance: z.boolean(),
                    cpu: z.int(),
                    memory: z.int(),
                    disk: z.int(),
                    allocated_cpu: z.int(),
                    allocated_memory: z.int(),
                    allocated_disk: z.int(),
                }),
            ),
        });

        const json = await this.rest.get("status");
        return schema.parse(json);
    }

    public async getState() {
        const schema = z.object({
            maintenance_mode: z.boolean(),
            available_free_servers: z.int().min(0).max(1000),
        });

        const json = await this.rest.get("state");
        return schema.parse(json);
    }

    public async patchState(data: { maintenance_mode?: boolean; available_free_servers?: number }) {
        await this.rest.voidPatch("state", data);
    }

    public async getUser(userId: string) {
        const schema = z.object({
            id: z.string(),
            panel_id: z.int(),
            discord_id: z.string(),
            last_login_at: z.int(),
            username: z.string(),
            credits: z.int(),
            referral_code: z.string(),
            referral_usage: z.int(),
            referral_gains: z.int(),
        });

        const json = await this.rest.get(`users/${userId}`);
        return schema.parse(json);
    }

    public async createCreditTransaction(
        userId: string,
        data: {
            type: CreditTransactionType;
            amount: number;
            reason: string | null;
        },
    ) {
        const schema = z.object({
            transaction_id: z.string(),
            user_credits: z.int(),
        });

        const json = await this.rest.post(`users/${userId}/credits`, data);
        return schema.parse(json);
    }

    public async createMassCreditTransaction(
        data: {
            discord_id: string;
            type: CreditTransactionType;
            amount: number;
            reason: string | null;
        }[],
    ) {
        await this.rest.post("credits", data);
    }
}
