import type {
    APIApplicationCommandOption,
    ApplicationIntegrationType,
    AutocompleteInteraction,
    ChatInputCommandInteraction,
    ContextMenuCommandInteraction,
    InteractionContextType,
    MessageComponentInteraction,
    ModalSubmitInteraction,
} from "discord.js";
import { config, colors, emojis } from "@/config/index";
import type { OrionBot } from "./bot";
import type { LocaleContext } from "./localizations/Localizations";

export interface BaseHandlerContext {
    lang: LocaleContext;
}

export type CommandHandlerContext = BaseHandlerContext & {
    interaction: ChatInputCommandInteraction;
};

export type ContextMenuHandlerContext = BaseHandlerContext & {
    interaction: ContextMenuCommandInteraction;
};

export type ComponentHandlerContext<
    T extends MessageComponentInteraction | ModalSubmitInteraction =
        | MessageComponentInteraction
        | ModalSubmitInteraction,
> = BaseHandlerContext & {
    interaction: T;
};

export type AutocompleteHandlerContext = BaseHandlerContext & {
    interaction: AutocompleteInteraction;
};

export interface CommandFile {
    default: {
        readonly data: CommandData;
        new (client: OrionBot, data: CommandData, filepath: string): Command;
    };
}

export enum CommandCategory {
    Informations,
    Private,
}

export type LocaleProperty = "name_localizations" | "description" | "description_localizations";
export type RemoveOptionLocalizations<T> = T extends { options?: (infer U)[] }
    ? Omit<T, LocaleProperty | "options"> & { description?: string; options?: RemoveOptionLocalizations<U>[] }
    : T extends { choices?: (infer U)[] }
      ? Omit<T, LocaleProperty | "choices"> & {
            description?: string;
            choices?: (Omit<U, "name"> & { name?: string } & { key?: string })[];
        }
      : Omit<T, LocaleProperty> & { description?: string };

export type UnlocalizedApplicationCommandOption = RemoveOptionLocalizations<APIApplicationCommandOption>;

export interface CommandData {
    name: string;
    category: CommandCategory;
    description?: string | null;
    integrationTypes?: ApplicationIntegrationType[] | null;
    contexts?: InteractionContextType[] | null;
    guilds?: string[] | null;
    ownerOnly?: boolean;
    enabled?: boolean;
    options?: UnlocalizedApplicationCommandOption[];
}

export abstract class Command implements CommandData {
    public static readonly data: CommandData;

    public readonly filepath: string;

    public readonly name: string;
    public readonly category: CommandCategory;
    public readonly description: string | null;
    public readonly integrationTypes: ApplicationIntegrationType[] | null;
    public readonly contexts: InteractionContextType[] | null;
    public readonly guilds: string[] | null;
    public readonly ownerOnly: boolean;
    public enabled: boolean;
    public readonly options: UnlocalizedApplicationCommandOption[];

    public readonly config: typeof config;
    public readonly colors: typeof colors;
    public readonly emojis: typeof emojis;

    public constructor(
        public readonly client: OrionBot<true>,
        data: CommandData,
        filepath: string,
    ) {
        this.filepath = filepath;

        this.name = data.name;
        this.category = data.category;
        this.description = data.description || null;
        this.integrationTypes = data.integrationTypes || null;
        this.contexts = data.contexts || null;
        this.guilds = data.guilds || null;
        this.ownerOnly = data.ownerOnly || false;
        this.enabled = data.enabled ?? true;
        this.options = data.options || [];

        this.config = config;
        this.colors = colors;
        this.emojis = emojis;
    }

    public handleCommand?(ctx: CommandHandlerContext): Promise<void>;
    public handleContextMenu?(ctx: ContextMenuHandlerContext): Promise<void>;
    public handleComponent?(ctx: ComponentHandlerContext): Promise<void>;
    public handleAutocomplete?(ctx: AutocompleteHandlerContext): Promise<void>;
}
