import type React from "react";

import type { Media } from "@lib/api/types";
import { useApi } from "@lib/api/context";

import { Image } from "./image";
import { buildClassname } from "@lib/ui/utils/classname";

export function MediaDisplay({
    media,
    className,
    style,
    fit = "contain",
}: { media: Media; fit?: "contain" | "cover" } & React.ComponentProps<"div">) {
    const { mediaApi } = useApi();

    const aspectRatio = (media.width ?? 1) / (media.height ?? 1);

    const fitProps: React.CSSProperties | undefined =
        fit == "contain"
            ? {
                  width: aspectRatio >= 1 ? "100%" : undefined,
                  height: aspectRatio <= 1 ? "100%" : undefined,
              }
            : undefined;

    return (
        <div
            className={buildClassname(className)}
            style={{ aspectRatio, ...fitProps, ...style }}
        >
            <Image
                src={mediaApi.getMediaUrl(media.id)}
                className="h-full"
                aspectRatio={aspectRatio}
            />
        </div>
    );
}
