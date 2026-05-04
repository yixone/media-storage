import { useState } from "react";

import type { Media } from "@/api/models";
import { useApi } from "@/providers";
import { cn } from "@/utils";

import { mediaAspectRatio } from "../media";

type AssetMediaProps = {
    media: Media.Media;
    useSkeleton?: boolean;
} & React.ComponentProps<"div">;

export function AssetMedia({
    media,
    useSkeleton = true,
    className,
    style,
    ...props
}: AssetMediaProps) {
    const aspectRatio = mediaAspectRatio(media);

    const [loaded, setLoaded] = useState(false);
    const { mediaApiV0 } = useApi();

    return (
        <div
            className={cn(className, aspectRatio > 1 ? "w-full" : "h-full")}
            style={{ aspectRatio, ...style }}
            {...props}
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
