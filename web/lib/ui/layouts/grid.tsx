import { useApi } from "@lib/api/context";
import type { Asset, Media } from "@lib/api/types";
import { useEffect, useState } from "react";
import { useResizeObserver } from "../observer";

const COLUMN_CALC_WIDTH = 250;
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
            grid
            p-2 gap-2
            overflow-hidden
            "
            ref={targetRef}
            style={{
                gridTemplateColumns: `repeat(${colsCount}, minmax(0, 1fr))`,
            }}
        >
            {assets.map((a) => (
                <GridAsset asset={a} key={a.id}>
                    <GridAssetMedia media={a.media} />
                    <GridAssetData title={a.title} />
                </GridAsset>
            ))}
        </div>
    );
}

/**
 * Container for the grid layout asset
 */
function GridAsset({
    asset,
    children,
}: React.ComponentProps<"div"> & { asset: Asset }) {
    return (
        <div className="block">
            <a
                className="
                hover:*:brightness-95 *:transition-[filter] *:duration-70
                flex flex-col gap-[0.15rem]
                "
                href={`/a/${asset.id}`}
            >
                {children}
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

    return (
        <div
            className="
            box-border aspect-square
            relative
            overflow-hidden
            rounded-[0.5rem]
            border border-border/75
            "
        >
            {!loaded && (
                <div
                    className="
                    size-full
                    absolute
                    animate-pulse
                    "
                    style={{ backgroundColor: `#${media.color}` }}
                />
            )}
            <img
                className="size-full object-cover"
                src={mediaApi.getMediaUrl(media.id)}
                onLoad={() => {
                    setLoaded(true);
                }}
                style={{
                    visibility: loaded ? "visible" : "hidden",
                }}
            />
        </div>
    );
}

/**
 * A component for displaying metadata for a grid layout asset
 */
function GridAssetData({ title }: { title: string | null }) {
    return (
        <p
            className="
            overflow-hidden text-ellipsis
            text-[1.125rem] text-primary
            "
        >
            {title}
        </p>
    );
}

export { AssetsGridLayout };
