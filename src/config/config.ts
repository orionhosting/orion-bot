const production = process.env.NODE_ENV === "production";

export default {
    hostingName: "Orion Hosting",
    ownerIds: ["619838036846575617", "755054105713704960"],
    supportGuildId: production ? "1306734190238371860" : "753715962720682004",
    aiChannelId: production ? "1307470895010549803" : "1430675822930432102",
    statusChannelId: production ? "1310373468423983135" : "1481021652555796711",
    supportInvite: "https://discord.gg/gzYKugxq9a",
    botInvite: "https://discord.com/oauth2/authorize?client_id=1306868952793747546",
    domain: "orionhost.xyz",
    domainURL: "https://orionhost.xyz",
    panelURL: "https://panel.orionhost.xyz",
    docsURL: "https://docs.orionhost.xyz",
    apiURL: "https://api.orionhost.xyz",
    statusURL: "https://status.orionhost.xyz",
    botAPIURL: "https://bot.orionhost.xyz",
    bannerURL:
        "https://media.discordapp.net/attachments/1480034193869246477/1480036557099503879/banner-large.png?format=webp&quality=lossless",
    ticketsPanelURL: "https://discord.com/channels/1306734190238371860/1307061230145507410/1413569386471489697",
};
