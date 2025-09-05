import type {
    APIApplicationCommandOption,
    AutocompleteInteraction,
    ChatInputCommandInteraction,
    ContextMenuCommandInteraction,
    MessageComponentInteraction,
    ModalSubmitInteraction,
} from "discord.js";
import { config, colors, emojis } from "@/config/index";
import type { LocaleContext } from "./localizations/Localizations";
import type { Pterobot } from "./Pterobot";

export interface CommandHandlerContext {
    lang: LocaleContext;
}

export type SlashHandlerContext = CommandHandlerContext & {
    interaction: ChatInputCommandInteraction<"cached">;
};

export type ContextMenuHandlerContext = CommandHandlerContext & {
    interaction: ContextMenuCommandInteraction<"cached">;
};

export type ComponentHandlerContext<
    T extends MessageComponentInteraction | ModalSubmitInteraction =
        | MessageComponentInteraction
        | ModalSubmitInteraction,
> = CommandHandlerContext & {
    interaction: T;
};

export type AutocompleteHandlerContext = CommandHandlerContext & {
    interaction: AutocompleteInteraction<"cached">;
};

export interface CommandFile {
    default: {
        readonly data: CommandData;
        new (client: Pterobot, data: CommandData, filepath: string): Command;
    };
}

export enum CommandCategory {
    Informations,
    Private,
}

export interface CommandData {
    name: string;
    category: CommandCategory;
    ownerOnly?: boolean;
    enabled?: boolean;
    options?: APIApplicationCommandOption[];
}

export abstract class Command implements CommandData {
    public static readonly data: CommandData;

    public readonly filepath: string;

    public readonly name: string;
    public readonly category: CommandCategory;
    public readonly ownerOnly: boolean;
    public enabled: boolean;
    public readonly options: APIApplicationCommandOption[];

    public readonly config: typeof config;
    public readonly colors: typeof colors;
    public readonly emojis: typeof emojis;

    public constructor(
        public readonly client: Pterobot<true>,
        data: CommandData,
        filepath: string,
    ) {
        this.filepath = filepath;

        this.name = data.name;
        this.category = data.category;
        this.ownerOnly = data.ownerOnly || false;
        this.enabled = data.enabled ?? true;
        this.options = data.options || [];

        this.config = config;
        this.colors = colors;
        this.emojis = emojis;
    }

    public handleSlash?(ctx: SlashHandlerContext): Promise<void>;
    public handleContextMenu?(ctx: ContextMenuHandlerContext): Promise<void>;
    public handleComponent?(ctx: ComponentHandlerContext): Promise<void>;
    public handleAutocomplete?(ctx: AutocompleteHandlerContext): Promise<void>;
}
