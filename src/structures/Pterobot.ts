import { readdirSync } from "node:fs";
import { join } from "node:path";
import type { Express } from "express";
import { Client, Collection, type ClientOptions } from "discord.js";
import { PlumeAPI } from "@sodiumlabs/plume-api";
import { DatabaseManager, type Manager } from "@/database/index";
import buildAPI from "@/api/index";
import type { Command, CommandFile } from "./command";
import { EventType, type Event, type EventFile } from "./event";
import RemoteLogger from "./RemoteLogger";
import ErrorManager from "./ErrorManager";

export class Pterobot<R extends boolean = boolean> extends Client<R> {
    public readonly db: DatabaseManager;
    public readonly remoteLogger: RemoteLogger;
    public readonly errors: ErrorManager;
    public readonly commands: Collection<string, Command>;
    public readonly events: Collection<string, Event>;
    public readonly api: Express;
    public readonly plumeAPI: PlumeAPI;
    private _manager: Manager | null = null;

    public constructor(options: ClientOptions) {
        super(options);

        this.db = new DatabaseManager(this);
        this.remoteLogger = new RemoteLogger(this);
        this.errors = new ErrorManager(this);
        this.commands = new Collection();
        this.events = new Collection();
        this.api = buildAPI();
        this.plumeAPI = new PlumeAPI();
    }

    public get manager() {
        if (!this._manager) throw new Error("No manager available");
        return this._manager;
    }

    public async loadCommands(): Promise<void> {
        const categoriesFolderPath = join(process.cwd(), "dist", "commands");
        for (const folder of readdirSync(categoriesFolderPath)) {
            const commandsFolderPath = join(categoriesFolderPath, folder);
            for (const file of readdirSync(commandsFolderPath)) {
                const filePath = join(commandsFolderPath, file);

                const ImportedCommand = ((await import(filePath)) as CommandFile).default;
                const command = new ImportedCommand(this, ImportedCommand.data, filePath);

                this.commands.set(command.name, command);
                console.log(`Loaded command ${command.name}`);
            }
        }
    }

    public async loadEvents(): Promise<void> {
        const categoriesFolderPath = join(process.cwd(), "dist", "events");
        for (const folder of readdirSync(categoriesFolderPath)) {
            const eventsFolderPath = join(categoriesFolderPath, folder);
            for (const file of readdirSync(eventsFolderPath)) {
                const filePath = join(eventsFolderPath, file);

                const ImportedEvent = ((await import(filePath)) as EventFile).default;
                const event = new ImportedEvent(this, ImportedEvent.data, filePath);

                this.events.set(event.name, event);

                const listen = (on: (name: string, listener: () => Promise<void>) => void): void => {
                    on(ImportedEvent.data.name, async (...args) => {
                        try {
                            if (!("handle" in event) || typeof event.handle !== "function") return;
                            await event.handle(...args);
                        } catch (err) {
                            this.remoteLogger.sendError("Event", err as Error);
                        }
                    });
                };

                switch (ImportedEvent.data.type) {
                    case EventType.Client: {
                        listen(this.on.bind(this));
                        break;
                    }
                    case EventType.Rest: {
                        listen(this.rest.on.bind(this.rest) as never);
                        break;
                    }
                    case EventType.Process: {
                        listen(process.on.bind(process));
                        break;
                    }
                }

                console.log(`Loaded event ${event.name}`);
            }
        }
    }

    public async start(): Promise<void> {
        await this.loadCommands();
        await this.loadEvents();
        // await this.db.connect();
        // this._manager = (await ManagerModel.findOne({})) || new ManagerModel();

        this.api.listen(Number(process.env.PORT), (...errs) => {
            if (errs.length) {
                console.error(errs);
            }

            console.log("API listening");
        });

        await this.login(process.env.DISCORD_TOKEN);
    }
}
