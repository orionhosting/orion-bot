import { readFileSync } from "node:fs";
import { join } from "node:path";
import i18next, { type TFunction, type TOptions } from "i18next";
import metadata, { type LocaleMetadata } from "./metadata";

export interface LocaleContext<Namespace extends string = string, Prefix extends string | undefined = undefined> {
    meta: LocaleMetadata;
    t: TFunction<Namespace, Prefix>;
    nt: <Opt extends TOptions>(key: string, options?: Opt) => ReturnType<TFunction<Namespace, Prefix>> | null;
    common: TFunction<"common", Prefix>;
    gt: TFunction<Namespace, Prefix>;
    ngt: <Opt extends TOptions>(key: string, options?: Opt) => ReturnType<TFunction<Namespace, Prefix>> | null;
    mapT: typeof Localizations.mapT;
    mapGT: typeof Localizations.mapT;
}

interface LocalizationOptions {
    onLog?: (message: string) => void;
    onError?: (message: string) => void;
}

type LocalizationResources = Record<string, object>;

interface ParsedLang {
    metadata: LocaleMetadata;
    translations: LocalizationResources;
}

export class Localizations {
    public static t = i18next.t;

    public static readonly defaultLang = "en-US";

    public static readonly namespaces = ["commands", "events", "common"];

    public static loadedLangs: string[] = [];

    public static loadedMeta: Record<string, LocaleMetadata> = {};

    public static options: LocalizationOptions | null = null;

    private static readonly _cachedContexts = new Map<string, LocaleContext>();

    public static initialize(options: LocalizationOptions) {
        Localizations.options = options;

        const parsed = Localizations._parseLangs();
        Localizations.loadedLangs = Object.keys(parsed);
        Localizations.loadedMeta = Object.fromEntries(Object.entries(parsed).map(([k, v]) => [k, v.metadata]));

        i18next.init({
            debug: process.env.NODE_ENV !== "production",
            initImmediate: false,
            returnNull: false,
            returnEmptyString: false,
            pluralSeparator: "+",
            contextSeparator: "%",
            interpolation: {
                escapeValue: false,
            },
            saveMissing: true,
            appendNamespaceToMissingKey: false,
            lng: Localizations.defaultLang,
            fallbackLng: Localizations.defaultLang,
            ns: Localizations.namespaces,
            resources: Object.fromEntries(Object.entries(parsed).map(([k, v]) => [k, v.translations])),
        });

        i18next.on("initialized", Localizations.onInitialized);
        i18next.on("loaded", Localizations.onLoaded);
        i18next.on("failedLoading", Localizations.onFailedLoading);
        i18next.on("missingKey", Localizations.onMissingKey);
        i18next.store.on("added", Localizations.onStoreAdded);
        i18next.store.on("removed", Localizations.onStoreRemoved);
    }

    public static mapT(key: string): Record<string, string> {
        const result: Record<string, string> = {};

        for (const lang of Localizations.loadedLangs) {
            result[lang] = Localizations.t(key, { lng: lang });
        }

        return result;
    }

    public static sanitizeLocale(key: string): string {
        return Localizations.loadedLangs.includes(key) ? key : Localizations.defaultLang;
    }

    public static getLocale<Namespace extends string = string, Prefix extends string | undefined = undefined>(
        lang: string,
        namespace?: Namespace,
        prefix?: Prefix,
    ): LocaleContext<Namespace, Prefix> {
        lang = Localizations.sanitizeLocale(lang);
        const key = `${lang}:${namespace}:${prefix}`;

        const stored = Localizations._cachedContexts.get(key);
        if (stored) return stored;

        const t = i18next.getFixedT(lang, namespace, prefix);
        const common = i18next.getFixedT(lang, "common");
        const gt = i18next.getFixedT(lang);

        const props: LocaleContext<Namespace, Prefix> = {
            meta: Localizations.loadedMeta[lang]!,
            t,
            nt: (key, options) => {
                if (i18next.exists(key)) {
                    const tr = t(key, options);
                    return tr;
                }
                return null;
            },
            common,
            gt,
            ngt: (key, options) => {
                if (i18next.exists(key)) {
                    const tr = gt(key, options);
                    return tr;
                }
                return null;
            },
            mapT: key => Localizations.mapT(`${namespace ? `${namespace}:` : ""}${prefix ? `${prefix}.` : ""}${key}`),
            mapGT: key => Localizations.mapT(key),
        };

        Localizations._cachedContexts.set(key, props);
        return props;
    }

    public static reloadLangs(langs?: string[] | null) {
        const oldLangs = Localizations.loadedLangs;
        const parsed = Localizations._parseLangs(langs);
        const newLangs = Object.keys(parsed);

        for (const lang of newLangs) {
            Localizations._log(`[${lang}] Reloading translations`);
            i18next.store.data[lang] = parsed[lang]!.translations;
            Localizations.loadedMeta[lang] = parsed[lang]!.metadata;
        }

        for (const lang of oldLangs.filter(l => !Localizations.loadedLangs.includes(l))) {
            if (!(lang in i18next.store.data)) continue;

            Localizations._log(`[${lang}] Deleting translations`);
            delete i18next.store.data[lang];
            delete Localizations.loadedMeta[lang];
        }

        Localizations.loadedLangs = newLangs;
    }

    private static onInitialized(): void {
        Localizations._log("Language System Initialized");
    }

    private static onLoaded(loaded: Record<string, Record<string, boolean>>): void {
        for (const lang of Object.keys(loaded)) {
            const namespaces = Object.keys(loaded[lang]!);
            const loadedNS = namespaces.filter(ns => loaded[lang]![ns]);
            const notLoadedNS = namespaces.filter(ns => !loaded[lang]![ns]);

            if (loadedNS) Localizations._log(`[${lang}] Loaded namespaces ${loadedNS.join(", ")}`);
            if (notLoadedNS) Localizations._log(`[${lang}] Unable to load namespaces ${notLoadedNS.join(", ")}`, true);
        }
    }

    private static onFailedLoading(lang: string, ns: string, msg: string): void {
        Localizations._log(`[${lang}] Failed to load namespace ${ns}: ${msg}`, true);
    }

    private static onMissingKey(langs: readonly string[], ns: string, key: string, res: string): void {
        Localizations._log(`[${langs.join(",")}] Key not found: "${ns}:${key}" (res: ${res})`, true);
    }

    private static onStoreAdded(lang: string, ns: string): void {
        Localizations._log(`[${lang}] Namespace added: ${ns}`);
    }

    private static onStoreRemoved(lang: string, ns: string): void {
        Localizations._log(`[${lang}] Namespace removed: ${ns}`);
    }

    private static _log(msg: string, isWarn?: boolean): void {
        if (isWarn) {
            Localizations.options?.onError?.(msg);
        } else {
            Localizations.options?.onLog?.(msg);
        }
    }

    private static _parseLangs(langs?: string[] | null): Record<string, ParsedLang> {
        const parsed: Record<string, ParsedLang> = {};

        for (const data of metadata) {
            if (langs && !langs.includes(data.locale)) continue;

            parsed[data.locale] = {
                metadata: data,
                translations: Object.fromEntries(
                    Localizations.namespaces.map(n => [
                        n,
                        JSON.parse(readFileSync(join("dist", "locales", data.name, `${n}.json`)).toString()),
                    ]),
                ),
            };
        }

        return parsed;
    }
}
