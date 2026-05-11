import { cn } from "@/utils";
import { calcAspectRatio } from "./aspect-ratio";
import type { Media } from "@/api/models";
import { useApi } from "@/providers";
import { useState } from "react";

export type MediaContainerProps = {
    media: Media.Media;

    className?: string;
    children?: React.ReactNode;
};

export function MediaContainer({
    media,
    className,
    children,
}: MediaContainerProps) {
    const aspectRatio = calcAspectRatio(media.width ?? 1, media.height ?? 1);

    return (
        <div
            className={cn(className, aspectRatio > 1 ? "w-full" : "h-full")}
            style={{ aspectRatio }}
        >
            <div
                className="size-full relative"
                style={{
                    backgroundColor: media.color ?? "#fff",
                }}
            >
                {children}
            </div>
        </div>
    );
}

export type MediaImageProps = {
    mediaId: string;
    format: Media.MediaFormat;

    className?: string;
    style?: React.CSSProperties;
    fetchPriority?: "auto" | "high" | "low";
    loading?: "lazy" | "eager";
};

export function MediaImage({
    mediaId,
    format,
    className,
    style,
    fetchPriority,
    loading,
}: MediaImageProps) {
    const { mediaApiV1 } = useApi();

    const [loaded, setLoaded] = useState(false);

    return (
        <img
            src={mediaApiV1.getMediaUrl(mediaId, format)}
            draggable={false}
            className={cn(
                "absolute top-0 left-0 size-full object-cover transition-opacity duration-125",
                className
            )}
            fetchPriority={fetchPriority}
            loading={loading}
            onLoad={() => {
                setLoaded(true);
            }}
            style={{
                opacity: loaded ? "100%" : "0%",
                ...style,
            }}
        />
    );
}
