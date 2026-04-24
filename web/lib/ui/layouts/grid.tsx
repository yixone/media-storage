import { useApi } from "@lib/api/context";
import type { Asset, Media } from "@lib/api/types";
import { useState } from "react";
import { useResizeObserver } from "../observer";

const COLUMN_CALC_WIDTH = 220;
const MIN_COLUMNS_COUNT = 2;

/**
 * Assets grid layout
 */
function AssetsGridLayout({ assets }: { assets: Asset[] }) {
    const calcColsCount = (rootWidth: number) => {
        return Math.floor(rootWidth / COLUMN_CALC_WIDTH);
    };

    const [colsCount, setColsCount] = useState(
        calcColsCount(window.innerWidth)
    );

    const { targetRef } = useResizeObserver((e) => {
        const newCount = Math.max(
            calcColsCount(e[0].contentRect.width),
            MIN_COLUMNS_COUNT
        );

        setColsCount(newCount);
    });

    return (
        <div
            className="
            grid gap-1
            overflow-hidden
            "
            ref={targetRef}
            style={{
                gridTemplateColumns: `repeat(${colsCount}, minmax(0, 1fr))`,
            }}
        >
            {assets.map((a) => (
                <GridAsset asset={a} key={a.id} />
            ))}
        </div>
    );
}

/**
 * Container for the grid layout asset
 */
function GridAsset({ asset }: { asset: Asset }) {
    return (
        <div className="block">
            <a
                className="
                hover:bg-border/45 transition-[background-color] duration-125
                rounded-md
                flex flex-col gap-2 items-center
                p-2
                "
                href={`/a/${asset.id}`}
            >
                <GridAssetMedia media={asset.media} />
                <GridAssetData title={asset.title} />
            </a>
        </div>
    );
}

/**
 * Media component for the grid layout asset
 */
function GridAssetMedia({ media }: { media: Media }) {
    const { mediaApi } = useApi();
    const [loaded, setLoaded] = useState(false);

    const aspectRatio = (media.width ?? 1) / (media.height ?? 1);

    return (
        <div
            className="
            aspect-square
            size-full
            box-border
            flex items-center justify-center
            "
        >
            <div
                className="
                overflow-hidden
                border border-border/50
                rounded-[0.5rem]
                relative
                "
                style={{
                    aspectRatio,
                    width: aspectRatio >= 1 ? "100%" : undefined,
                    height: aspectRatio <= 1 ? "100%" : undefined,
                }}
            >
                <img
                    className="
                    object-cover size-full
                    transition-opacity duration-125
                    "
                    src={mediaApi.getMediaUrl(media.id)}
                    onLoad={() => {
                        setLoaded(true);
                    }}
                    style={{
                        opacity: loaded ? "100%" : "0%",
                    }}
                />
            </div>
        </div>
    );
}

/**
 * A component for displaying metadata for a grid layout asset
 */
function GridAssetData({ title }: { title: string | null }) {
    return (
        <div className="w-[75%]">
            <p
                className="
                overflow-hidden text-ellipsis
                whitespace-nowrap
                text-[1.125rem] text-primary/80
                "
            >
                {title}
            </p>
        </div>
    );
}

export { AssetsGridLayout };
