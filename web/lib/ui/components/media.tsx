import { useApi } from "@lib/api/context";
import type { Media } from "@lib/api/types";
import { buildClassname } from "./utils";
import { useState } from "react";
import React from "react";

type MediaHolderContextProps = {
    media: Media;

    loaded: boolean;
    setLoaded: (loaded: boolean) => void;
};

const MediaHolderContext = React.createContext<MediaHolderContextProps | null>(
    null
);

function useMediaContext() {
    const ctx = React.useContext(MediaHolderContext);
    if (!ctx) {
        throw new Error(
            "useMediaContext() hook must be used within an MediaHolderContext"
        );
    }

    return ctx;
}

/**
 * Media holder component
 */
function MediaHolder({
    media,
    className,
    children,
    ...props
}: { media: Media } & React.ComponentProps<"img">) {
    const [loaded, setLoaded] = useState(false);
    const ctx = React.useMemo(
        () => ({
            media: media,
            loaded,
            setLoaded: (loaded: boolean) => {
                setLoaded(loaded);
            },
        }),
        [loaded, media]
    );

    const aspectRatio = (media.width ?? 1) / (media.height ?? 1);

    return (
        <MediaHolderContext.Provider value={ctx}>
            <div
                className={buildClassname("block relative", className)}
                style={{ aspectRatio, ...props.style }}
            >
                {children}
            </div>
        </MediaHolderContext.Provider>
    );
}

function MediaContent({
    className,
    onLoad,
    ...props
}: React.ComponentProps<"img">) {
    const { mediaApi } = useApi();
    const { media, loaded, setLoaded } = useMediaContext();

    return (
        <img
            className={buildClassname("size-full", className)}
            src={mediaApi.getMediaUrl(media.id)}
            style={{
                visibility: loaded ? "visible" : "hidden",
                ...props.style,
            }}
            onLoad={(e) => {
                if (onLoad) onLoad(e);
                setLoaded(true);
            }}
            {...props}
        />
    );
}

function MediaSkeleton({ className, ...props }: React.ComponentProps<"div">) {
    const { media, loaded } = useMediaContext();
    if (loaded) return;

    return (
        <div
            className={buildClassname(
                "absolute size-full animate-pulse",
                className
            )}
            style={{ backgroundColor: `#${media.color}` }}
            {...props}
        />
    );
}

export { MediaHolder, MediaContent, MediaSkeleton };
