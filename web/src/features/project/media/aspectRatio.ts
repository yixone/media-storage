import type { Media } from "./models";

export function getAspectRatio(media: Media) {
    return (media.width ?? 1) / (media.height ?? 1);
}
