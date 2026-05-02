import type { Media } from "@/api/models";

export function aspectRatio(width: number, height: number) {
    return width / height;
}

export function mediaAspectRatio(media: Media.Media) {
    return aspectRatio(media.width ?? 1, media.height ?? 1);
}
