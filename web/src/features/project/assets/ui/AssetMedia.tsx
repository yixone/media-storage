import { useState } from "react";

import { useApi } from "@/api/context";
import { buildClassname } from "@/ui/utils/classname";

import type { Media } from "../../media/models";
import { getAspectRatio } from "../../media/utils";

type AssetMediaProps = {
    media: Media;
    useSkeleton?: boolean;
    className?: string;
};

export function AssetMedia({
    media,
    useSkeleton = true,
    className,
}: AssetMediaProps) {
    const aspectRatio = getAspectRatio(media);

    const [loaded, setLoaded] = useState(false);
    const { mediaApi } = useApi();

    return (
        <div
            className={buildClassname(
                className,
                aspectRatio > 1 ? "w-full" : "h-full"
            )}
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
                    src={mediaApi.getMediaUrl(media.id)}
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
