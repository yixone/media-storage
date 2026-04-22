import type { Media } from "./Media";

/**
 * Asset API model data
 */
export type AssetItem = {
    id: string;
    media: Media;
    created_at: string;
    title: string | null;
    caption: string | null;
    source_url: string | null;
};

/**
 * Asset API model
 */
export class Asset {
    item: AssetItem;

    constructor(item: AssetItem) {
        this.item = item;
    }
}

/**
 * Class for interacting with the Asset API
 */
export class AssetApi {}
