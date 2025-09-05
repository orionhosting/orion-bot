import { Locale } from "discord.js";

const metadata = [
    {
        id: 0,
        locale: Locale.EnglishUS,
        name: "en-US",
    },
    {
        id: 1,
        locale: Locale.French,
        name: "fr-FR",
    },
];

export type LocaleMetadata = (typeof metadata)[number];

export default metadata;
