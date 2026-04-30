import { useEffect, useState } from "react";

import { Grid } from "@/ui/Grid";

import { useInspector } from "@/features/common/inspector";

import { useNavigate } from "react-router";
import { AssetMedia } from "./AssetMedia";
import type { Asset } from "../models";
import { getAssetViewUrl } from "../utils/url";

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
        <Grid columnWidth={230}>
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
    const navigate = useNavigate();

    const goToAssetPage = () => {
        navigate(getAssetViewUrl(asset));
    };

    return (
        <div
            data-selected={selected}
            className="
            group/grid-asset 
            cursor-pointer 
            transition-[background-color] duration-125 
            bg-transparent hover:bg-border/25  data-[selected=true]:bg-foreground/8
            rounded-md p-1
            flex flex-col
            focus:outline-none
            "
            onClick={() => onSelect(asset)}
            onFocus={() => onSelect(asset)}
            onDoubleClick={() => goToAssetPage()}
            tabIndex={1}
        >
            <div className="aspect-square overflow-hidden flex justify-center items-center">
                <AssetMedia
                    className="border border-border/60 overflow-hidden rounded-md"
                    media={asset.media}
                />
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
