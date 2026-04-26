import { useState } from "react";

import type { Asset, Media } from "@lib/api/types";

import { useResizeObserver } from "@lib/ui/utils/observer";
import { buildClassname } from "@lib/ui/utils/classname";

import { useInspector } from "../../inspector";
import { MediaDisplay } from "../media/MediaDisplay";

const COLUMN_CALC_WIDTH = 250;
const MIN_COLUMNS_COUNT = 2;

/**
 * Assets grid layout
 */
function AssetsGrid({ assets }: { assets: Asset[] }) {
    const calcColsCount = (rootWidth: number) => {
        console.log(rootWidth);
        return Math.floor(rootWidth / COLUMN_CALC_WIDTH);
    };

    const { targetRef } = useResizeObserver((e) => {
        const newCount = Math.max(
            calcColsCount(e[0].contentRect.width),
            MIN_COLUMNS_COUNT
        );
        setColsCount(newCount);
    });

    const [colsCount, setColsCount] = useState(
        calcColsCount(window.innerWidth)
    );

    return (
        <div
            className="
            grid gap-1 overflow-hidden w-full
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
    // FIXME: Each asset item personally tracks the asset selected in the inspector
    const { displayAsset, selectedAsset } = useInspector();
    const assetSelected = selectedAsset === asset;

    return (
        <div className="block">
            <div
                className={buildClassname(
                    `
                transition-[background-color] duration-125 
                rounded-md
                flex flex-col gap-2 items-center
                p-2
                cursor-pointer
                `,
                    assetSelected ? "bg-foreground/8" : "hover:bg-foreground/5"
                )}
                onClick={() => displayAsset(asset)}
            >
                <GridAssetMedia media={asset.media} selected={assetSelected} />
                <GridAssetData title={asset.title} />
            </div>
        </div>
    );
}

/**
 * Media component for the grid layout asset
 */
function GridAssetMedia({
    media,
    selected,
}: {
    media: Media;
    selected: boolean;
}) {
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
            <MediaDisplay
                media={media}
                className={buildClassname(
                    "overflow-hidden border rounded-[0.5rem]",
                    selected
                        ? "outline-2 outline-foreground border-foreground"
                        : "border-border/50"
                )}
                style={{
                    width: aspectRatio >= 1 ? "100%" : undefined,
                    height: aspectRatio <= 1 ? "100%" : undefined,
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
                text-[1.125rem] text-primary/80
                text-center
                "
            >
                {title}
            </p>
        </div>
    );
}

export { AssetsGrid };
