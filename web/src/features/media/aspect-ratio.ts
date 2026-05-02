import type { Media } from "@/api/models";

export function calcAspectRatio(width: number, height: number) {
    return width / height;
}

export function mediaAspectRatio(media: Media.Media) {
    return calcAspectRatio(media.width ?? 1, media.height ?? 1);
}
