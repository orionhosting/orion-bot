import mongoose from "mongoose";
import type { Pterobot } from "@/structures/index";
import { saveDocument } from "./saveDocument";

export class DatabaseManager {
    public save = saveDocument;

    public readonly pingData: {
        retryTimeout: number;
        lastTimestamp: number | null;
        last: number | null;
    } = {
        retryTimeout: 15000,
        lastTimestamp: null,
        last: null,
    };

    public constructor(public readonly client: Pterobot) {
        mongoose.connection.on("connecting", () => console.debug("[Mongoose]", "Connecting to MongoDB server..."));
        mongoose.connection.on("connected", () => {
            console.info("[Mongoose]", "Connected to MongoDB server");
        });
        mongoose.connection.on("disconnecting", () => {
            console.debug("[Mongoose]", "Connection#close() has been called, disconnecting from MongoDB...");
        });
        mongoose.connection.on("disconnected", () => {
            console.error("[Mongoose]", "Connection losted with MongoDB server");
        });
        mongoose.connection.on("reconnected", () => {
            console.info("[Mongoose]", "Successfully reconnected to MongoDB after losted connectivity");
        });
        mongoose.connection.on("error", err => console.error("[Mongoose]", "An error occured", err));
        mongoose.connection.on("reconnectFailed", err =>
            console.error("[Mongoose]", "Failed to reconnect to the MongoDB server", err),
        );
    }

    public async connect(): Promise<void> {
        await mongoose.connect(process.env.MONGODB_URI, {
            autoIndex: false,
            serverSelectionTimeoutMS: 5000,
            socketTimeoutMS: 45000,
            family: 4,
        });
    }

    public async ping(): Promise<number | null> {
        if (Date.now() - (this.pingData.lastTimestamp || 0) < this.pingData.retryTimeout) return this.pingData.last;

        try {
            const startTimestamp = Date.now();
            await mongoose.connection.db?.admin().ping();

            this.pingData.lastTimestamp = Date.now();
            this.pingData.last = this.pingData.lastTimestamp - startTimestamp;

            return this.pingData.last;
        } catch (err) {
            this.client.errors.create(__filename, "Failed to ping MongoDB server", err as Error);

            this.pingData.lastTimestamp = Date.now();
            return null;
        }
    }
}
