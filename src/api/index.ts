import assert from "node:assert";
import fastifyRatelimit from "@fastify/rate-limit";
import fastifySwagger from "@fastify/swagger";
import fastifySwaggerUI from "@fastify/swagger-ui";
import { Time } from "@voctal/duration";
import Fastify from "fastify";
import {
    hasZodFastifySchemaValidationErrors,
    isResponseSerializationError,
    jsonSchemaTransform,
    jsonSchemaTransformObject,
    serializerCompiler,
    validatorCompiler,
    ZodTypeProvider,
} from "fastify-type-provider-zod";
import z from "zod";
import { version } from "@/../package.json";
import { prisma } from "@/database/prisma";
import { OrionBot } from "@/structures";

let swagger: ReturnType<ReturnType<typeof Fastify>["swagger"]> | null = null;

export const startAPI = async (client: OrionBot) => {
    const fastify = await createFastify(client);

    fastify.get(
        "/api/ping",
        {
            schema: {
                tags: ["General"],
                summary: "Ping the API",
            },
        },
        async (_, reply) => reply.send(),
    );

    fastify.get(
        "/api/users/:id/boosts",
        {
            schema: {
                tags: ["Users"],
                summary: "Get the boosting state of the user",
                headers: z.object({
                    authorization: z.string().max(100),
                }),
                params: z.object({ id: z.string().max(20) }),
                response: {
                    200: z.object({ active_since: z.int().nullable(), next_reward_at: z.int().nullable() }),
                    401: z.void(),
                    404: z.void(),
                },
            },
        },
        async (req, reply) => {
            if (req.headers.authorization !== process.env.ADMIN_API_TOKEN) {
                return reply.code(401).send();
            }

            const state = await prisma.userBoostState.findUnique({
                where: {
                    user_id: req.params.id,
                },
            });
            if (!state) {
                return reply.code(404).send();
            }

            return reply.send({
                active_since: state.boosting_since,
                next_reward_at: state.boosting_since ? client.services.boosts.getNextRewardTimestamp(state) : null,
            });
        },
    );

    fastify.get(
        "/api/users/:id/ad",
        {
            schema: {
                tags: ["Users"],
                summary: "Get the ad state of the user",
                params: z.object({ id: z.string().max(20) }),
                response: {
                    200: z.array(
                        z.object({
                            guild_id: z.string(),
                            guild_name: z.string(),
                            active_since: z.int(),
                            next_reward_at: z.int(),
                        }),
                    ),
                    401: z.void(),
                },
            },
        },
        async (req, reply) => {
            if (req.headers.authorization !== process.env.ADMIN_API_TOKEN) {
                return reply.code(401).send();
            }

            const guilds = client.guilds.cache.filter(g => g.ownerId === req.params.id);
            if (!guilds.size) {
                return reply.send([]);
            }

            const states = await prisma.guildAdState.findMany({
                where: {
                    guild_id: { in: guilds.map(g => g.id) },
                },
            });
            if (!states.length) {
                return reply.send([]);
            }

            const values = [];
            for (const state of states) {
                if (!state.valid_since) continue;

                const guild = guilds.find(g => g.id === state.guild_id);
                assert(guild !== undefined, "guild is missing");

                values.push({
                    guild_id: state.guild_id,
                    guild_name: guild.name,
                    active_since: state.valid_since,
                    next_reward_at:
                        (!state.last_reward_at || state.valid_since > state.last_reward_at
                            ? state.valid_since
                            : state.last_reward_at) + Time.Week,
                });
            }
        },
    );

    await fastify.ready();
    swagger = fastify.swagger();

    await fastify.listen({ host: "0.0.0.0", port: Number(process.env.PORT) });
};

const createFastify = async (client: OrionBot) => {
    const fastify = Fastify({
        routerOptions: {
            ignoreTrailingSlash: true,
        },
        loggerInstance: client.logger,
    }).withTypeProvider<ZodTypeProvider>();

    fastify.setValidatorCompiler(validatorCompiler);
    fastify.setSerializerCompiler(serializerCompiler);

    fastify.setNotFoundHandler(async (_, reply) => {
        return reply.code(404).send({ message: "Not found" });
    });

    fastify.setErrorHandler((err, req, reply) => {
        if (hasZodFastifySchemaValidationErrors(err)) {
            return reply.code(422).send({
                message: "Request doesn't match the schema",
                errors: err.validation.map(e => ({
                    path: `${err.validationContext}${e.instancePath}`,
                    message: e.message,
                })),
            });
        }

        if (isResponseSerializationError(err)) {
            req.log.error(err, "Response doesn't match the schema");
            return reply.code(500).send({
                message: "Response doesn't match the schema",
            });
        }

        req.log.error(err);
        return reply.code(500).send({ message: "An unknown error occured" });
    });

    await fastify.register(fastifyRatelimit, {
        max: 300,
        timeWindow: 60_000,
    });

    // OpenAPI

    await fastify.register(fastifySwagger, {
        openapi: {
            openapi: "3.1.0",
            servers: [
                {
                    url: `${process.env.NODE_ENV === "production" ? "https://bot.orionhost.xyz" : `http://localhost:${process.env.PORT}`}/api`,
                    description: "The base API URL",
                },
            ],
            info: {
                title: "Orion Bot API",
                version,
            },
        },
        transform: jsonSchemaTransform,
        transformObject: jsonSchemaTransformObject,
    });

    await fastify.register(fastifySwaggerUI, {
        routePrefix: "/docs",
        theme: { title: "API Documentation" },
    });

    fastify.get("/api/docs/openapi.json", (_, reply) => reply.send(swagger));

    return fastify;
};
