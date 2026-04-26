import type { Media } from "./media";

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
 * Asset creation DTO
 */
export type CreateAssetData = {
    attachment: File;
    title: string | null;
    caption: string | null;
    source_url: string | null;
};
