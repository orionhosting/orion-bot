<div align="center">
    <img src="https://raw.githubusercontent.com/octarahq/orion-hosting/refs/heads/main/brand-assets/orion-logo-rounded.png" width="128" alt="Orion Logo">
    <h1>Orion Bot</h1>
    <p>
        <a href="https://orionhost.xyz/discord"><img src="https://img.shields.io/discord/1306734190238371860?color=5865F2&logo=discord&logoColor=white" alt="Discord server" /></a>
        <a href="https://github.com/orionhosting/orion-bot/commits/main"><img alt="Last commit" src="https://img.shields.io/github/last-commit/orionhosting/orion-bot?logo=github&logoColor=ffffff" /></a>
    </p>
</div>

## About

The source code of Orion Bot. The bot is present on the support but can be added to any account/server to access its commands anywhere.

The bot uses [Twilight](https://github.com/twilight-rs/twilight) for the Discord client, [Rig](https://github.com/0xplaygrounds/rig) for the chatbot, [Axum](https://github.com/tokio-rs/axum) for the API, and [rust-intl](https://github.com/voctal/rust-intl) for translations.

The bot was originally written in TypeScript using discord.js, but was rewritten to reduce resource usage (RAM: ~120MB -> ~20MB, disk: ~700MB -> ~30MB). View old code [here](https://github.com/orionhosting/orion-bot/tree/776d7a4d7bb216e197fda7e719b2293a75f732d2).

## Note

- The hardcoded bot config has some Discord links, they need to be kept in sync.

- The bot uses Orion API and the docs API, and Orion API also uses the bot API.

- [dev] The `orion-api` crate is temporary until the official Rust API client is released.

- [dev] Ideally the `framework` crate should be moved to its own repo and be framework-agnostic (serenity/twilight) with a `twilight` feature (if feasible).

- [dev] The `locales` folder will have to be moved to the future `intl` repo.

## API

The API follows the [OpenAPI v3.1 specification](https://spec.openapis.org/oas/v3.1.0.html). The API documentation can be found at `/docs`, and the JSON specification file at `/api/docs/openapi.json`.

### Ratelimits headers

- `x-ratelimit-limit` - Request limit
- `x-ratelimit-remaining` - The number of requests left for the time window
- `x-ratelimit-after` - Number of seconds in which the API will become available after its rate limit has been exceeded (only present on 429)

## Deployment

- `npm i -g @orionhosting/cli` (install the CLI if not already done)
- `bash deploy.bash` (deploy to Orion)
