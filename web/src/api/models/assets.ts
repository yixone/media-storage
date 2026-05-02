import type { Media } from "./media";

export namespace Assets {
    export type Asset = {
        id: string;
        media: Media.Media;
        created_at: string;
        title: string | null;
        caption: string | null;
        source_url: string | null;
    };

    export type CreateAssetRequest = {
        attachment: File;
        title: string | null;
        caption: string | null;
        source_url: string | null;
    };
}
