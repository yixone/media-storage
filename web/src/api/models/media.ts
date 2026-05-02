export namespace Media {
    export type MediaState = "Pending" | "Processing" | "Ready" | "Failed";

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
}
