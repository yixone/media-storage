import { useApi } from "@lib/api/context";
import type { Asset, Media } from "@lib/api/types";
import { useState } from "react";
import { useResizeObserver } from "../observer";

const COLUMN_CALC_WIDTH = 200;
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
                hover:*:brightness-93 *:transition-[filter] *:duration-70
                flex flex-col gap-[0.15rem] items-center
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

    return (
        <div
            className="
            box-border aspect-square
            relative
            flex items-center justify-center
            "
        >
            <img
                className="
                max-h-full max-w-full
                overflow-hidden
                rounded-[0.5rem]
                border border-border/75
                "
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
        <div className="w-[75%]">
            <p
                className="
            overflow-hidden text-ellipsis
            whitespace-nowrap
            text-[1.125rem] text-primary
            "
            >
                {title}
            </p>
        </div>
    );
}

export { AssetsGridLayout };
