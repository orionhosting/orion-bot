import { sep } from "node:path";
import type { Pterobot } from "./Pterobot";

export default class ErrorManager {
    public constructor(public readonly client: Pterobot) {}

    public create(filepath: string, origin: string, error: unknown): void {
        this.register("error", filepath, origin, error, true);
    }

    private register(type: "warn" | "error", filepath: string, origin: string, error: unknown, send: boolean) {
        const filename = filepath.split(sep).at(-1);

        if (type === "error") {
            if (error === null) throw new Error("error is null");

            console.error("ErrorHandler", filename, origin, error);
            if (send)
                this.client.remoteLogger.sendError(
                    `[${filename}] ${origin}`,
                    error instanceof Error ? error : new Error(`${error}`),
                );
        }
    }
}
