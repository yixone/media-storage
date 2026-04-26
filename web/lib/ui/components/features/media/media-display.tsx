import React, { useState } from "react";

import type { Media } from "@lib/api/types";
import { useApi } from "@lib/api/context";
import { buildClassname } from "@lib/ui/utils/classname";

export function MediaDisplay({
    media,
    className,
    ...props
}: {
    media: Media;
} & React.ComponentProps<"div">) {
    const [loaded, setLoaded] = useState(false);
    const aspectRatio = (media.width ?? 1) / (media.height ?? 1);

    const { mediaApi } = useApi();

    return (
        <div
            className={buildClassname("relative", className)}
            style={{
                aspectRatio,
                backgroundColor: `#${media.color}`,
                ...props.style,
            }}
        >
            <img
                className="size-full transition-opacity duration-150"
                src={mediaApi.getMediaUrl(media.id)}
                style={{ opacity: loaded ? "100%" : "0%" }}
                onLoad={() => setLoaded(true)}
                loading="lazy"
                fetchPriority="high"
            />
        </div>
    );

    //
}
