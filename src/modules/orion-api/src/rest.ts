import { OrionAPIClient } from "..";
import { OrionAPIError } from "./errors";

export interface RequestOptions {
    method?: string;
    path: string;
    body?: unknown;
}

export class REST {
    public readonly apiURL: string;

    public constructor(public readonly client: OrionAPIClient) {
        this.apiURL = `${client.url}/api`;
    }

    public async request({ method, path, body }: RequestOptions): Promise<Response> {
        const object: Record<string, string> = {
            Authorization: `${this.client.key}`,
        };
        if (body) {
            object["Content-Type"] = "application/json";
        }
        const response = await fetch(`${this.apiURL}/${path}`, {
            method: method || "GET",
            headers: object,
            body: body ? JSON.stringify(body) : undefined,
        });

        if (!response.ok) {
            let message;
            try {
                const error = await response.json();
                if (
                    typeof error === "object" &&
                    error !== null &&
                    "message" in error &&
                    typeof error.message === "string"
                ) {
                    message = error.message;
                }
            } catch {}
            throw new OrionAPIError({ message, response });
        }

        return response;
    }

    public async get(path: string): Promise<unknown> {
        const response = await this.request({ path });
        const data: unknown = await response.json();
        return data;
    }

    public async post(path: string, body: object): Promise<unknown> {
        const response = await this.request({ method: "POST", path, body });
        const data: unknown = await response.json();
        return data;
    }

    public async patch(path: string, body: object): Promise<unknown> {
        const response = await this.request({ method: "PATCH", path, body });
        const data: unknown = await response.json();
        return data;
    }

    public async voidPatch(path: string, body: object): Promise<unknown> {
        const response = await this.request({ method: "PATCH", path, body });
        return response;
    }

    public async voidPost(path: string): Promise<Response> {
        const response = await this.request({ method: "POST", path });
        return response;
    }

    public async voidDelete(path: string): Promise<Response> {
        const response = await this.request({ path });
        return response;
    }
}
