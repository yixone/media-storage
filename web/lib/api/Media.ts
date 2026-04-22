/**
 * Media API model data
 */
export type MediaItem = {
    id: string;
    state: MediaState;
    created_at: string;
    size: number;
    mimetype: string;
    width: number | null;
    height: number | null;
    color: string | null;
};

/**
 * Media State
 */
export type MediaState = "Pending" | "Processing" | "Ready" | "Failed";

/**
 * Media API model
 */
export class Media {
    item: MediaItem;

    constructor(item: MediaItem) {
        this.item = item;
    }
}

/**
 * Class for interacting with the Media API
 */
export class MediaApi {}
