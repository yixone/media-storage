import { useEffect, useState } from "react";

import type { Asset } from "@lib/api/types";

import { buildClassname } from "@lib/ui/classname";
import { MediaDisplay } from "@lib/ui/media";
import { Grid } from "@lib/ui/components/base";

import { useInspector } from "../../../providers";

const COLUMN_CALC_WIDTH = 240;

/**
 * Assets grid layout
 */
function AssetsGrid({ assets }: { assets: Asset[] }) {
    const selectAsset = (asset: Asset) => {
        setSelectedId(asset.id);
        addView({ type: "display.asset", asset });
    };

    const [selectedId, setSelectedId] = useState<string | null>(null);
    const { addView } = useInspector();

    useEffect(() => {
        if (assets.length === 0) return;
        selectAsset(assets[0]);
    }, [assets]);

    return (
        <Grid columnWidth={COLUMN_CALC_WIDTH}>
            {assets
                .filter((a) => a.media.state === "Ready")
                .map((a) => (
                    <GridAsset
                        asset={a}
                        key={a.id}
                        selected={a.id === selectedId}
                        onSelect={selectAsset}
                    />
                ))}
        </Grid>
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
            focus:outline-none
            "
            onClick={() => onSelect(asset)}
            tabIndex={1}
            onFocus={() => onSelect(asset)}
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
