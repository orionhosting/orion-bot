import {
    ApplicationCommandOptionType,
    Events,
    type ChatInputCommandInteraction,
    type CommandInteractionOption,
    type Interaction,
} from "discord.js";
import { Localizations, Event, EventType, type EventData } from "@/structures/index";

export default class extends Event {
    public static override readonly data: EventData = {
        name: Events.InteractionCreate,
        type: EventType.Client,
    };

    public async handle(interaction: Interaction): Promise<void> {
        const lang = Localizations.getLocale(interaction.locale, "events", "interactionCreate");

        if (interaction.isChatInputCommand()) {
            const command = this.client.commands.get(interaction.commandName);
            if (!command) return this.client.logger.warn(`Command ${interaction.commandName} not found`);
            if (typeof command.handleCommand !== "function") return;

            if (!command.enabled) {
                await interaction.reply({
                    content: `${this.emojis.warn} ${lang.t("err_command_disabled")}`,
                    ephemeral: true,
                });
                return;
            }
            if (command.ownerOnly && !this.config.ownerIds.includes(interaction.user.id)) {
                await interaction.reply({
                    content: `${this.emojis.warn} ${lang.t("err_missing_permissions")}`,
                    ephemeral: true,
                });
                return;
            }

            try {
                this.client.logger.info(
                    `[Command] ${interaction.commandName}: ${interaction.user.username} (${interaction.user.id})`,
                );
                this.client.remoteLogger.sendInfo(
                    "log",
                    `\`${interaction.user.username} (${interaction.user.id})\` has used \`/${this._getFullCommandInput(interaction)}\``,
                );

                await command.handleCommand({
                    interaction,
                    lang: Localizations.getLocale(interaction.locale, "commands", command.name),
                });
            } catch (err) {
                this.client.monitor.captureException(err, "Command");

                if (!interaction.replied && !interaction.deferred) {
                    await interaction.reply({
                        content: `${this.emojis.warn} ${lang.common("errors.err_unknown")}`,
                        ephemeral: true,
                    });
                }
            }
            return;
        }

        if (interaction.isContextMenuCommand()) {
            const command = this.client.commands.get(interaction.commandName);
            if (!command) return this.client.logger.warn(`ContextMenu ${interaction.commandName} not found`);
            if (typeof command.handleContextMenu !== "function") return;

            if (!command.enabled) {
                await interaction.reply({
                    content: `${this.emojis.warn} ${lang.t("err_command_disabled")}`,
                    ephemeral: true,
                });
                return;
            }
            if (command.ownerOnly && !this.config.ownerIds.includes(interaction.user.id)) {
                await interaction.reply({
                    content: `${this.emojis.warn} ${lang.t("err_missing_permissions")}`,
                    ephemeral: true,
                });
                return;
            }

            try {
                this.client.logger.info(
                    `[ContextMenu] ${interaction.commandName}: ${interaction.user.username} (${interaction.user.id})`,
                );
                await command.handleContextMenu({
                    interaction,
                    lang: Localizations.getLocale(interaction.locale, "commands", command.name),
                });
            } catch (err) {
                this.client.monitor.captureException(err, "Command");
            }
            return;
        }

        if (interaction.isMessageComponent() || interaction.isModalSubmit()) {
            const commandName = interaction.customId.split("-")[0];
            const command = commandName && this.client.commands.get(commandName);

            if (!command) return;
            if (typeof command.handleComponent !== "function") return;

            if (!command.enabled) return;
            if (command.ownerOnly && !this.config.ownerIds.includes(interaction.user.id)) return;

            try {
                await command.handleComponent({
                    interaction,
                    lang: Localizations.getLocale(interaction.locale, "commands", command.name),
                });
            } catch (err) {
                this.client.monitor.captureException(err, "Command");
            }
            return;
        }

        if (interaction.isAutocomplete()) {
            const command = this.client.commands.get(interaction.commandName);
            if (!command) return;
            if (typeof command.handleAutocomplete !== "function") return;

            if (!command.enabled) return;
            if (command.ownerOnly && !this.config.ownerIds.includes(interaction.user.id)) return;

            try {
                await command.handleAutocomplete({
                    interaction,
                    lang: Localizations.getLocale(interaction.locale, "commands", command.name),
                });
            } catch (err) {
                this.client.monitor.captureException(err, "Command");
            }
            return;
        }
    }

    private _getFullCommandInput(interaction: ChatInputCommandInteraction): string {
        const fillLogs = (logs: string[], options: readonly CommandInteractionOption[]): string[] => {
            const firstOption = options[0];
            if (!firstOption) return logs;

            if (
                ![ApplicationCommandOptionType.SubcommandGroup, ApplicationCommandOptionType.Subcommand].includes(
                    firstOption.type,
                )
            ) {
                for (const o of options) {
                    logs.push(`${o.name}:${o.value}`);
                }
                return logs;
            }

            logs.push(firstOption.name);
            if (firstOption.options) return fillLogs(logs, firstOption.options);

            return logs;
        };

        return fillLogs([interaction.commandName], interaction.options.data).join(" ");
    }
}
