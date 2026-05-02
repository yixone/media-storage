import { useState } from "react";

import { useApi } from "@/providers";
import { cn } from "@/utils/classname";

import type { Media } from "@/api/models";
import { mediaAspectRatio } from "@/features/media";

type AssetMediaProps = {
    media: Media.Media;
    useSkeleton?: boolean;
    className?: string;
};

export function AssetMedia({
    media,
    useSkeleton = true,
    className,
}: AssetMediaProps) {
    const aspectRatio = mediaAspectRatio(media);

    const [loaded, setLoaded] = useState(false);
    const { mediaApiV0 } = useApi();

    return (
        <div
            className={cn(className, aspectRatio > 1 ? "w-full" : "h-full")}
            style={{ aspectRatio }}
        >
            <div
                className="size-full relative"
                style={{
                    backgroundColor: useSkeleton
                        ? `#${media.color ?? "fff"}`
                        : undefined,
                    animation: loaded ? undefined : "var(--animate-pulse)",
                }}
            >
                <img
                    src={mediaApiV0.getMediaUrl(media.id)}
                    draggable={false}
                    className="absolute top-0 left-0 size-full"
                    fetchPriority="high"
                    loading="lazy"
                    onLoad={() => setLoaded(true)}
                    style={{
                        visibility: loaded ? "visible" : "hidden",
                    }}
                />
            </div>
        </div>
    );
}
