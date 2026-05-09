export namespace Media {
    export type MediaStatus = "Pending" | "Processing" | "Ready" | "Failed";

    export type Media = {
        id: string;
        status: MediaStatus;
        created_at: string;
        blob_size: number;
        content_type: string;
        width: number | null;
        height: number | null;
        color: string | null;
    };
}
