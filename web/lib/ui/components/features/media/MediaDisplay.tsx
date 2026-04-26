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
            style={{ aspectRatio, ...props.style }}
        >
            {!loaded && (
                <div
                    className="absolute size-full animate-pulse"
                    style={{ backgroundColor: `#${media.color}` }}
                />
            )}

            <img
                className="size-full"
                src={mediaApi.getMediaUrl(media.id)}
                style={{ visibility: loaded ? "visible" : "hidden" }}
                onLoad={() => setLoaded(true)}
            />
        </div>
    );

    //
}
