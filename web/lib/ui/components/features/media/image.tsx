import type React from "react";

import { buildClassname } from "@lib/ui/utils/classname";

export function Image({
    src,
    aspectRatio = 1,
    className,
    ...props
}: {
    src: string;
    aspectRatio?: number;
    skeletonColor?: string;
} & React.ComponentProps<"img">) {
    return (
        <div className={buildClassname(className)} style={{ aspectRatio }}>
            <img
                src={src}
                className="w-full max-w-full h-auto"
                fetchPriority="high"
                loading="lazy"
                {...props}
            />
        </div>
    );
}
