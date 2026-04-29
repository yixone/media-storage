import React, { useState } from "react";

import type { Media } from "@lib/api/types";
import { useApi } from "@lib/api/context";

import { buildClassname } from "@lib/ui/classname";

export const MediaDisplay = React.memo(
    ({
        media,
        className,
        style,
    }: { media: Media } & React.ComponentProps<"div">) => {
        const [loaded, setLoaded] = useState(false);
        const { mediaApi } = useApi();

        return (
            <div
                className={buildClassname(className, "relative")}
                style={{
                    aspectRatio: (media.width ?? 1) / (media.height ?? 1),
                    backgroundColor: `#${media.color}`,
                    animation: loaded ? undefined : "var(--animate-pulse)",
                    ...style,
                }}
            >
                <img
                    src={mediaApi.getMediaUrl(media.id)}
                    draggable={false}
                    className="absolute w-full h-auto max-h-full transition-opacity duration-125 select-none"
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
);
