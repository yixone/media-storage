/**
 * Media State
 */
export type MediaState = "Pending" | "Processing" | "Ready" | "Failed";

/**
 * Media model
 */
export type Media = {
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
 * Media creation DTO
 */
export type CreateMediaData = {
    file: File;
};
