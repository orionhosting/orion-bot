import assert from "node:assert";
import { readdirSync } from "node:fs";
import { join } from "node:path";
import { PlumeAPI } from "@voctal/plume-api";
import { Client, Collection, type ClientOptions } from "discord.js";
import pino, { Logger } from "pino";
import { startAPI } from "@/api";
import { config } from "@/config";
import { OrionAPIClient } from "@/modules/orion-api";
import { Cache } from "./cache";
import type { Command, CommandFile } from "./command";
import { EventType, type Event, type EventFile } from "./event";
import { Localizations } from "./localizations/Localizations";
import Monitor from "./Monitor";
import RemoteLogger from "./RemoteLogger";
import ServiceManager from "./services/service-manager";

/**
 * The Orion Bot main client.
 */
export class OrionBot<R extends boolean = boolean> extends Client<R> {
    public readonly logger: Logger;
    public readonly remoteLogger: RemoteLogger;
    public readonly monitor: Monitor;
    public readonly commands: Collection<string, Command>;
    public readonly events: Collection<string, Event>;
    public readonly plumeAPI: PlumeAPI;
    public readonly services: ServiceManager;
    public readonly cache: Cache;
    public readonly orionAPI: OrionAPIClient;

    public constructor(options: ClientOptions) {
        super(options);

        this.logger = pino({
            level: "debug",
            transport: {
                targets: [
                    {
                        target: "pino-pretty",
                        options: {
                            colorize: true,
                            ignore: "pid,hostname",
                            translateTime: "HH:MM:ss.l",
                        },
                    },
                    {
                        target: "pino/file",
                        options: {
                            destination: join("logs", "app.log"),
                            mkdir: true,
                        },
                    },
                ],
            },
        });
        this.remoteLogger = new RemoteLogger(this);
        this.monitor = new Monitor(this);
        this.cache = new Cache();
        this.commands = new Collection();
        this.events = new Collection();
        this.plumeAPI = new PlumeAPI();
        this.orionAPI = new OrionAPIClient({ url: config.apiURL, key: process.env.ADMIN_API_TOKEN });
        this.services = new ServiceManager(this);
    }

    /**
     * Get a locale context.
     */
    public getLocale = Localizations.getLocale;

    /**
     * Get the bot support guild.
     */
    public getSupportGuild() {
        const guild = this.guilds.cache.get(config.supportGuildId);
        assert(guild, "support guild not found");
        return guild;
    }

    /**
     * Load all commands from the `commands` folder.
     */
    public async loadCommands(): Promise<void> {
        const categoriesFolderPath = join(process.cwd(), "dist", "commands");
        for (const folder of readdirSync(categoriesFolderPath)) {
            const commandsFolderPath = join(categoriesFolderPath, folder);
            for (const file of readdirSync(commandsFolderPath)) {
                const filePath = join(commandsFolderPath, file);

                const ImportedCommand = ((await import(filePath)) as CommandFile).default;
                const command = new ImportedCommand(this, ImportedCommand.data, filePath);

                this.commands.set(command.name, command);
                this.logger.info(`Loaded command ${command.name}`);
            }
        }
    }

    /**
     * Load and listen to all events from the `events` folder.
     */
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
                            this.monitor.captureException(err, "Event");
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

                this.logger.info(`Loaded event ${event.name}`);
            }
        }
    }

    /**
     * Start the bot and all services.
     */
    public async start(): Promise<void> {
        await Promise.all([this.loadCommands(), this.loadEvents()]);

        await this.login(process.env.DISCORD_TOKEN);
        await startAPI(this);

        this.logger.info("Bot started");
    }
}
