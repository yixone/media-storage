import { useState } from "react";

import type { Media } from "@lib/api/types";
import { useApi } from "@lib/api/context";
import { buildClassname } from "@lib/ui/classname";

type AssetMediaProps = {
    media: Media;
    className?: string;
};

export function AssetMedia({ media, className }: AssetMediaProps) {
    const aspectRatio = (media.width ?? 1) / (media.height ?? 1);
    const isHorizontal = aspectRatio > 1;

    return (
        <div
            className={buildClassname("relative size-full", className)}
            style={{ aspectRatio }}
        >
            <div className="grid size-full top-0 left-0 absolute overflow-hidden">
                <div
                    className={buildClassname(
                        isHorizontal
                            ? "w-full self-center"
                            : "h-full justify-self-center"
                    )}
                    style={{ aspectRatio }}
                >
                    <div className="overflow-hidden rounded-md border border-border/50">
                        <MediaContainer
                            aspectRatio={aspectRatio}
                            color={media.color ?? "fff"}
                            mediaId={media.id}
                        />
                    </div>
                </div>
            </div>
        </div>
    );
}

type MediaContainerProps = {
    aspectRatio: number;
    color: string;
    mediaId: string;
};

function MediaContainer({ aspectRatio, color, mediaId }: MediaContainerProps) {
    const [loaded, setLoaded] = useState(false);
    const { mediaApi } = useApi();

    return (
        <div
            className="relative"
            style={{
                aspectRatio,
                backgroundColor: `#${color}`,
                animation: loaded ? undefined : "var(--animate-pulse)",
            }}
        >
            <img
                src={mediaApi.getMediaUrl(mediaId)}
                draggable={false}
                className="absolute top-0 left-0 size-full transition-opacity duration-125 select-none"
                fetchPriority="high"
                loading="lazy"
                onLoad={() => setLoaded(true)}
                style={{
                    opacity: loaded ? "100%" : "0%",
                }}
            />
        </div>
    );
}
