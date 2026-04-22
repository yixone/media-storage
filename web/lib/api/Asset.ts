import { AbstractApi } from "./client";
import type { Media } from "./Media";

/**
 * Asset model
 */
export type Asset = {
    id: string;
    media: Media;
    created_at: string;
    title: string | null;
    caption: string | null;
    source_url: string | null;
};

/**
 * Asset API client
 */
export class AssetApi extends AbstractApi {}
