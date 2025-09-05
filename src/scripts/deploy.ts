import "dotenv/config";
import { readdirSync } from "node:fs";
import { join } from "node:path";
import { ApplicationCommandType, REST, Routes } from "discord.js";
import { Localizations, type CommandFile } from "@/structures/index";

Localizations.initialize({
    onLog: (message: string) => console.log(message),
    onError: (message: string) => console.error(message),
});

const deploy = async () => {
    const commands = [];

    const categoriesFolderPath = join(process.cwd(), "dist", "commands");
    for (const folder of readdirSync(categoriesFolderPath)) {
        const commandsFolderPath = join(categoriesFolderPath, folder);
        for (const file of readdirSync(commandsFolderPath)) {
            const filePath = join(commandsFolderPath, file);

            const ImportedCommand = ((await import(filePath)) as CommandFile).default;
            const data = ImportedCommand.data;

            commands.push({
                name: data.name,
                type: ApplicationCommandType.ChatInput,
                description: Localizations.t(`commands:${data.name}.description`),
                description_localizations: Localizations.mapT(`commands:${data.name}.description`),
                options: data.options,
            });
        }
    }

    const rest = new REST({ version: "10" }).setToken(process.env.DISCORD_TOKEN);

    try {
        await rest.put(Routes.applicationCommands(process.env.DISCORD_ID), {
            body: commands,
        });

        console.log("Deployed");
    } catch (err) {
        console.error(err);
    }
};

deploy();
