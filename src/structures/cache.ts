import { ValueCache } from "@voctal/cache";
import { config } from "@/config";

/**
 * The Orion documentation sitemap.
 */
type Sitemap = {
    url: string;
    lang: string;
    name: string;
    description: string;
}[];

/**
 * Global cache.
 */
export class Cache {
    /**
     * Cache of the documentation sitemap.
     */
    private readonly documentationSitemapCache = new ValueCache<Sitemap>({ ttl: 600_000 });

    /**
     * Get the cached Orion documentation sitemap.
     */
    public async getDocumentationSitemap() {
        const cached = this.documentationSitemapCache.get();
        if (cached) return cached;

        const res = await fetch(`${config.docsURL}/api/pages`);
        const sitemap = (await res.json()) as Sitemap;

        this.documentationSitemapCache.set(sitemap);
        return sitemap;
    }
}
