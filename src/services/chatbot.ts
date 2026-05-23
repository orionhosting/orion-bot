import { setTimeout as sleep } from "node:timers/promises";
import {
    GoogleGenerativeAI,
    GoogleGenerativeAIFetchError,
    SchemaType,
    type GenerativeModel,
} from "@google/generative-ai";
import { Cache } from "@voctal/cache";
import {
    ActionRowBuilder,
    ButtonBuilder,
    ButtonStyle,
    ChannelType,
    GuildTextBasedChannel,
    MessageFlags,
    MessageType,
    PermissionFlagsBits,
    TextDisplayBuilder,
    type Message,
} from "discord.js";
import { DateTime } from "luxon";
import type { OrionBot } from "@/structures/index";
import { Service } from "@/structures/services/service";

export class ChatbotService extends Service {
    public override readonly data = {
        name: "chatbot",
    };

    // Google settings
    public readonly googleGenerativeAI: GoogleGenerativeAI;
    public readonly gemini: GenerativeModel;

    public constructor(client: OrionBot) {
        super(client);

        this.googleGenerativeAI = new GoogleGenerativeAI(process.env.GEMINI_KEY);
        this.gemini = this.googleGenerativeAI.getGenerativeModel({
            model: "gemini-2.5-flash",
            tools: [
                {
                    functionDeclarations: [
                        {
                            name: "fetch_docs",
                            description: "Get the raw MDX of a page from the Orion documentation website.",
                            parameters: {
                                type: SchemaType.OBJECT,
                                properties: {
                                    url: {
                                        type: SchemaType.STRING,
                                        description: `The URL of the page to get. ('${this.config.docsURL}' followed by the path, like /fr/...)`,
                                    },
                                },
                                required: ["url"],
                            },
                        },
                    ],
                },
            ],
        });
    }

    public readonly globalMaxCredits = 1000;
    public readonly timeBetweenMessages = 1500;

    public readonly chattingGuilds = new Set<string>();
    public readonly credits = new Cache<"global", number>();

    private lastRatelimitAlert = 0;

    public async onMessage(message: Message<true>): Promise<void> {
        if (message.channelId !== this.config.aiChannelId) return;

        // Conditions
        if (message.author.bot || message.webhookId) return;
        if (message.type !== MessageType.Default && message.type !== MessageType.Reply) return;
        if (message.channel.type !== ChannelType.GuildText) return;

        if (message.type === MessageType.Reply) {
            if (message.mentions.repliedUser) {
                if (message.mentions.repliedUser.id !== this.client.user.id) {
                    return;
                }

                const hasMention = message.mentions.has(message.mentions.repliedUser.id);
                if (!hasMention) return;
            }
        }

        if (!this._hasChannelPermission(message.channel)) return;

        // Check the credits and if it is already chatting
        if (this.chattingGuilds.has(message.guildId)) return;

        const globalCredits = this.credits.get("global") || 0;
        if (globalCredits >= this.globalMaxCredits) return;

        // Reset the credit at the end of the day
        const tomorrow = DateTime.now().plus({ days: 1 }).startOf("day").toMillis();
        this.credits.set("global", globalCredits + 1, tomorrow - Date.now());

        this.chattingGuilds.add(message.guildId);

        // Generate
        try {
            await message.channel.sendTyping();
        } catch {
            // TODO: failed to send typing
        }

        try {
            await this._generateMessage(message);
        } catch (err) {
            this.client.monitor.captureException(err, `Failed to generate a chatbot message in ${message.guildId}`);
        } finally {
            await sleep(this.timeBetweenMessages);
            this.chattingGuilds.delete(message.guildId);
        }
    }

    private async _generateMessage(message: Message<true>): Promise<void> {
        // History

        const history = [];
        let historyLength = 0;

        for (const msg of [...message.channel.messages.cache.values()].reverse()) {
            if (!msg.content.length) continue;
            if (msg.id === message.id) continue;

            // The history is limited to 1500 caracters. However, we include at least one message
            if (historyLength > 0) {
                if (historyLength + msg.content.length >= 1500) break;
            }

            history.unshift({
                text: `@${msg.author.username}: ${msg.content}`,
                role: msg.author.id === this.client.user.id ? "model" : "user",
            });

            if (history.length >= 10) break;
            historyLength += msg.content.length;
        }

        // First content should be from the user (Google error otherwise)
        while (history.length && history[0]?.role === "model") {
            history.splice(0, 1);
        }

        // Instructions

        const sitemap = await this.client.cache.getDocumentationSitemap();
        const systemInstruction = `You are a Discord bot called 'Orion Hosting'. You are currently on its support server, in the AI discussion channel.

Orion Hosting is a free hosting platform made by 'Octara' (octara.xyz) and 'Voctal' (voctal.dev). (Made in collaboration).

You need to help the users asking questions about the hosting platform.

The users can ask about Orion or anything related to programming/IT, otherwise kindly explain that you are not made to reply to that.

If you are missing information to respond, tell the user to open a ticket on the discord server.

Do NOT do [link](link) where both texts inside [] and () are the same. Just put the link without the []().

Only if they asks, the docs website is made using Fumadocs, a doc framework for Next.js.

If 'Ayham' talks to you and is trolling you, he is probably farming credits (messages = credits rewards), tell him to stop farming credits

# Links

- https://orionhost.xyz - The website (presentation, about, tos, etc.)

- https://orionhost.xyz/dashboard - The dashboard, to create your account and your free servers

- https://panel.orionhost.xyz - The panel (Uses Pelican panel, fork of Pterodactyl)

- https://status.orionhost.xyz - The status page for our services

- https://docs.orionhost.xyz - The documentation for our services

- https://github.com/voctal/orion-cli - The CLI to deploy from the terminal

# Ports

The servers have multiple adresses:

- http://fr1.orionhost.xyz:4xxx - HTTP public port
- https://4xxx.fr1.orionhost.xyz - HTTPS url (uses the same number as the public port)
- Custom *.orionhost.app subdomain configurable in the dashboard

# Docs (fr)

You can fetch the docs using the fetch_docs tool. Here are the pages urls:

${sitemap
    .filter(s => s.lang === "fr")
    .map(s => `- [${s.name}](${s.url.slice(this.config.docsURL.length)}): ${s.description}`)
    .join("\n\n")}
`;

        // Chat

        const chat = this.gemini.startChat({
            systemInstruction: { parts: [{ text: systemInstruction }], role: "model" },
            history: history.map(h => ({
                parts: [{ text: h.text }],
                role: h.role,
            })),
        });

        let response;
        try {
            const result = await chat.sendMessage(`@${message.author.username}: ${message.content}`);
            response = result.response;
        } catch (err) {
            if (err instanceof GoogleGenerativeAIFetchError) {
                if (err.status === 503) {
                    await message.react("⏰");
                    return;
                } else if (err.status === 429) {
                    if (Date.now() - this.lastRatelimitAlert > 60_000) {
                        this.lastRatelimitAlert = Date.now();
                        try {
                            await message.reply(
                                "*The chatbot has reached its message limit. Please wait a moment or contact the support instead.*",
                            );
                        } catch {}
                    }
                    return;
                } else {
                    throw err;
                }
            } else {
                throw err;
            }
        }

        let sourcePage;
        let responseMessage;

        const call = response.candidates?.[0]?.content.parts.find(p => p.functionCall);
        if (call?.functionCall) {
            const { name, args } = call.functionCall;
            if (name !== "fetch_docs") throw new Error(`Invalid function call name: ${name}`);
            if (!("url" in args) || typeof args.url !== "string")
                throw new Error(`Invalid function args: ${JSON.stringify(args)}`);

            const sitemap = await this.client.cache.getDocumentationSitemap();
            sourcePage = sitemap.find(s => s.url === args.url);
            if (!sourcePage) {
                this.client.logger.warn(`Invalid url from Gemini: ${args.url}`);
                return;
            }

            responseMessage = await message.reply({
                embeds: [
                    {
                        color: this.colors.green.int,
                        description: `${this.emojis.loaderGreen} Reading the documentation...`,
                    },
                ],
            });

            const res = await fetch(`${sourcePage.url}.mdx`);
            const text = await res.text();

            const result = await chat.sendMessage([
                {
                    functionResponse: {
                        name,
                        response: {
                            text,
                        },
                    },
                },
            ]);
            response = result.response;
        }

        // Return

        let text = response.text() || "No response";
        // Remove mention at the start if he added one
        text = text.replace(/^@[\w.]+:\s*/g, "");
        text = text.replace(/^@Orion Hosting:/, "");
        text = text.replace(/^Content: /, "");

        const sourceButton = sourcePage
            ? new ButtonBuilder()
                  .setEmoji({ id: this.emojis.propertyId })
                  .setLabel(`Source - ${sourcePage.name}`)
                  .setStyle(ButtonStyle.Link)
                  .setURL(sourcePage.url)
            : null;
        const rows = sourceButton ? [new ActionRowBuilder<ButtonBuilder>().setComponents(sourceButton)] : [];

        if (text.length > 2000) {
            if (responseMessage?.deletable) await responseMessage.delete();

            await message.reply({
                components: [new TextDisplayBuilder().setContent(text.slice(0, 4000)), ...rows],
                flags: MessageFlags.IsComponentsV2,
                allowedMentions: {
                    parse: [],
                },
            });
        } else {
            const respond = responseMessage ? responseMessage.edit.bind(responseMessage) : message.reply.bind(message);

            await respond({
                content: text,
                components: rows,
                embeds: [],
                allowedMentions: {
                    parse: [],
                },
            });
        }
    }

    private _hasChannelPermission(channel: GuildTextBasedChannel): boolean {
        if (
            !channel.guild.members.me
                ?.permissionsIn(channel)
                .has(
                    PermissionFlagsBits.ViewChannel | PermissionFlagsBits.SendMessages | PermissionFlagsBits.EmbedLinks,
                )
        )
            return false;
        if (channel.guild.members.me.isCommunicationDisabled()) return false;
        return true;
    }
}
