import "dotenv/config";
import { readdirSync } from "node:fs";
import { join } from "node:path";
import {
    APIApplicationCommandOption,
    ApplicationCommandOptionType,
    ApplicationCommandType,
    ChatInputApplicationCommandData,
    REST,
    Routes,
} from "discord.js";
import {
    LocaleContext,
    Localizations,
    UnlocalizedApplicationCommandOption,
    type CommandFile,
} from "@/structures/index";

Localizations.initialize({
    onLog: (message: string) => console.log(message),
    onError: (message: string) => console.error(message),
});

const deploy = async () => {
    const commands = new Map<string, ChatInputApplicationCommandData[]>();
    const addCommand = (guildId: string, command: ChatInputApplicationCommandData) => {
        if (commands.has(guildId)) {
            commands.get(guildId)?.push(command);
        } else {
            commands.set(guildId, [command]);
        }
    };

    const categoriesFolderPath = join(process.cwd(), "dist", "commands");
    for (const folder of readdirSync(categoriesFolderPath)) {
        const commandsFolderPath = join(categoriesFolderPath, folder);
        for (const file of readdirSync(commandsFolderPath)) {
            const filePath = join(commandsFolderPath, file);

            const ImportedCommand = ((await import(filePath)) as CommandFile).default;
            const data = ImportedCommand.data;

            const lang = Localizations.getLocale(Localizations.defaultLang, "commands", data.name);

            const command: ChatInputApplicationCommandData = {
                name: data.name,
                type: ApplicationCommandType.ChatInput,
                integration_types: data.integrationTypes || undefined,
                contexts: data.contexts || undefined,
                description: data.description || Localizations.t(`commands:${data.name}.description`),
                description_localizations: data.description
                    ? undefined
                    : Localizations.mapT(`commands:${data.name}.description`),
                // @ts-expect-error
                options: data.options?.map(opt => buildOption(lang, [opt.name], opt)),
            };

            if (data.guilds) {
                for (const id of data.guilds) {
                    addCommand(id, command);
                }
            } else {
                addCommand("global", command);
            }
        }
    }

    const rest = new REST({ version: "10" }).setToken(process.env.DISCORD_TOKEN);

    try {
        await rest.put(Routes.applicationCommands(process.env.DISCORD_ID), {
            body: commands.get("global"),
        });

        console.log("Deployed globally");
    } catch (err) {
        console.error(err);
    }

    for (const [key, value] of commands.entries()) {
        if (key === "global") continue;

        try {
            await rest.put(Routes.applicationGuildCommands(process.env.DISCORD_ID, key), {
                body: value,
            });

            console.log(`Deployed in ${key}`);
        } catch (err) {
            console.error(err);
        }
    }
};

export const buildOption = (
    lang: LocaleContext,
    segments: string[],
    option: UnlocalizedApplicationCommandOption,
): APIApplicationCommandOption => {
    const newOption = {
        ...option,
        description: option.description || lang.t(`options.${segments.join(".")}.description`),
        description_localizations: option.description
            ? undefined
            : lang.mapT(`options.${segments.join(".")}.description`),
    } as APIApplicationCommandOption;

    if ("choices" in option) {
        // @ts-expect-error newOption is too hard to type
        newOption.choices = option.choices?.map(c => {
            const global = c.key?.includes(":");

            return {
                value: c.value,
                name: c.name || (c.key ? (global ? lang.gt(c.key) : lang.t(c.key)) : undefined),
                name_localizations:
                    c.name_localizations || (c.key ? (global ? lang.mapGT(c.key) : lang.mapT(c.key)) : undefined),
            };
        });
    }

    if (
        (option.type === ApplicationCommandOptionType.Subcommand ||
            option.type === ApplicationCommandOptionType.SubcommandGroup) &&
        option.options
    ) {
        // @ts-expect-error newOption is too hard to type
        newOption.options = option.options.map(opt => buildOption(lang, [...segments, opt.name], opt));
    }

    return newOption;
};

deploy();
