import { useEffect, useState } from "react";

import type { Asset } from "@lib/api/types";

import { useResizeObserver } from "@lib/ui/observer";
import { buildClassname } from "@lib/ui/classname";

import { useInspector } from "../../../providers";
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

    const selectAsset = (asset: Asset) => {
        setSelectedId(asset.id);
        addView({ type: "display.asset", asset });
    };

    const [gridReady, setGridReady] = useState(false);
    const [colsCount, setColsCount] = useState(MIN_COLUMNS_COUNT);

    const [selectedId, setSelectedId] = useState<string | null>(null);

    const { addView } = useInspector();

    useEffect(() => {
        if (assets.length === 0) return;
        selectAsset(assets[0]);
    }, [assets]);

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
                    .map((a) => {
                        return (
                            <GridAsset
                                asset={a}
                                key={a.id}
                                selected={a.id === selectedId}
                                onSelect={selectAsset}
                            />
                        );
                    })}
        </div>
    );
}

/**
 * Container for the grid layout asset
 */
function GridAsset({
    asset,
    selected,
    onSelect,
}: {
    asset: Asset;
    selected: boolean;
    onSelect: (asset: Asset) => void;
}) {
    const aspectRatio = (asset.media.width ?? 1) / (asset.media.height ?? 1);

    return (
        <div
            data-selected={selected}
            className="
            group/grid-asset 
            cursor-pointer 
            transition-[background-color] duration-125 
            bg-transparent hover:bg-border/25  data-[selected=true]:bg-foreground/8
            rounded-xl p-1
            "
            onClick={() => onSelect(asset)}
        >
            <div className="aspect-square flex items-center justify-center overflow-hidden p-2">
                <div
                    className={buildClassname(
                        aspectRatio >= 1 ? "w-full" : "h-full"
                    )}
                    style={{
                        aspectRatio,
                    }}
                >
                    <div
                        className="
                    border border-border/65 
                    overflow-hidden 
                    rounded-md 
                    group-hover/grid-asset:border-ring/65 
                    group-data-[selected=true]/grid-asset:border-ring 
                    group-data-[selected=true]/grid-asset:outline 
                    group-data-[selected=true]/grid-asset:outline-ring
                    "
                    >
                        <MediaDisplay media={asset.media} />
                    </div>
                </div>
            </div>
            <div className="w-full px-8">
                <p className="overflow-hidden text-ellipsis whitespace-nowrap text-[1.125rem] text-primary/90 text-center">
                    {asset.title}
                </p>
            </div>
        </div>
    );
}

export { AssetsGrid };
