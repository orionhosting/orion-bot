import { inspect } from "node:util";
import { Time } from "@voctal/duration";
import { OrionBot } from "./bot";

/**
 * Client monitor. Mainly manages the application exceptions.
 *
 * @warn Too many exceptions in a short time span will result in an auto shutdown to prevent more damages.
 */
export default class Monitor {
    public shutdownExceptionCount = 0;
    public shutdownExceptionStartTime: number | null = null;
    public readonly shutdownExceptionLimit = 50;
    public readonly shutdownResetInterval = Time.Hour;

    public constructor(public readonly client: OrionBot) {}

    public captureMessage(message: string): void {
        this.client.remoteLogger.sendWarning(`[MONITOR] ${message}`);
    }

    public captureException(exception: unknown, message?: string): void {
        if (exception instanceof Error) {
            this.client.remoteLogger.sendError(`Captured exception${message ? ` | ${message}` : ""}`, exception);
        } else {
            const trace: { stack?: string } = {};
            Error.captureStackTrace(trace);
            const origin = trace.stack?.split("\n")[1]?.trim();

            this.client.remoteLogger.sendWarning(
                `Captured exception (${origin})${message ? ` | ${message}` : ""}: ${inspect(exception)}`,
            );
        }

        this._incrementShutdownExceptions();
    }

    private _incrementShutdownExceptions() {
        if (
            !this.shutdownExceptionStartTime ||
            Date.now() - this.shutdownExceptionStartTime > this.shutdownResetInterval
        ) {
            this.shutdownExceptionStartTime = Date.now();
            this.shutdownExceptionCount = 0;
        }

        this.shutdownExceptionCount++;

        if (this.shutdownExceptionCount >= this.shutdownExceptionLimit) {
            console.log("Too many errors, exiting the process...");
            process.nextTick(() => process.exit(1));
        }
    }
}
