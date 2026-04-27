import { useState } from "react";

import type { Asset, Media } from "@lib/api/types";

import { useResizeObserver } from "@lib/ui/utils/observer";
import { buildClassname } from "@lib/ui/utils/classname";

import { useInspector } from "../../inspector";
import { MediaDisplay } from "../media";

const COLUMN_CALC_WIDTH = 275;
const MIN_COLUMNS_COUNT = 2;

/**
 * Assets grid layout
 */
function AssetsGrid({ assets }: { assets: Asset[] }) {
    const calcColsCount = (rootWidth: number) => {
        const cols = Math.max(
            Math.floor(rootWidth / COLUMN_CALC_WIDTH),
            MIN_COLUMNS_COUNT
        );
        return cols;
    };

    const { targetRef } = useResizeObserver((e) => {
        const newCount = calcColsCount(e[0].contentRect.width);
        setColsCount(newCount);

        if (!gridReady) {
            setGridReady(true);
        }
    });

    const [gridReady, setGridReady] = useState(false);
    const [colsCount, setColsCount] = useState(MIN_COLUMNS_COUNT);

    return (
        <div
            className="grid gap-1 overflow-hidden w-full"
            ref={targetRef}
            style={{
                gridTemplateColumns: `repeat(${colsCount}, minmax(0, 1fr))`,
            }}
        >
            {gridReady &&
                assets
                    .filter((a) => a.media.state === "Ready")
                    .map((a) => <GridAsset asset={a} key={a.id} />)}
        </div>
    );
}

/**
 * Container for the grid layout asset
 */
function GridAsset({ asset }: { asset: Asset }) {
    // FIXME: Each asset item personally tracks the asset selected in the inspector
    const { push } = useInspector();

    return (
        <div
            className={buildClassname("")}
            onClick={() => push({ type: "display_asset", asset })}
        >
            <GridAssetMedia media={asset.media} />
            <GridAssetData title={asset.title} />
        </div>
    );
}

/**
 * Media component for the grid layout asset
 */
function GridAssetMedia({ media }: { media: Media }) {
    const aspectRatio = (media.width ?? 1) / (media.height ?? 1);
    return (
        <div className="aspect-square flex items-center justify-center overflow-hidden p-4">
            <div
                className={buildClassname(
                    "justify-self-center",
                    aspectRatio >= 1 ? "w-full" : "h-full"
                )}
                style={{
                    aspectRatio,
                }}
            >
                <div className="border border-border/65 overflow-hidden rounded-md">
                    <MediaDisplay media={media} />
                </div>
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
                text-center
                "
            >
                {title}
            </p>
        </div>
    );
}

export { AssetsGrid };
