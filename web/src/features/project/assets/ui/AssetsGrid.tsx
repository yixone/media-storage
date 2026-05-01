import { useEffect, useState } from "react";
import { useNavigate } from "react-router";

import { useInspector } from "@/features/common/inspector";

import { AssetMedia } from "./AssetMedia";
import type { Asset } from "../models";
import { getAssetViewUrl } from "../utils/url";
import {
    useIntersectionObserver,
    useResizeObserver,
} from "@/features/common/observer";

function useGridLayout() {
    const [colsCount, setColsCount] = useState(2);
    const [layoutReady, setLayoutReady] = useState(false);

    const calcColsCount = (rootWidth: number) => {
        const cols = Math.max(Math.floor(rootWidth / 230), 2);
        return cols;
    };

    const { targetRef } = useResizeObserver((e) => {
        const newCount = calcColsCount(e[0].contentRect.width);
        setColsCount(newCount);

        if (!layoutReady) {
            setLayoutReady(true);
        }
    });

    return { colsCount, layoutReady, targetRef };
}

function useListRefill(refillCallback: () => void) {
    const { targetRef } = useIntersectionObserver(
        (e) => {
            const entry = e[0];
            if (!entry.isIntersecting) return;
            refillCallback();
        },
        {
            threshold: 0.15,
            rootMargin: "15px",
        }
    );

    return { refillTriggerRef: targetRef };
}

function useAssetActions(assets: Asset[]) {
    const [selectedId, setSelectedId] = useState<string | null>(null);
    const { addView } = useInspector();

    const navigate = useNavigate();

    const selectAsset = (asset: Asset) => {
        setSelectedId(asset.id);
        addView({ type: "display.asset", asset });
    };

    const openAsset = (asset: Asset) => {
        navigate(getAssetViewUrl(asset));
    };

    useEffect(() => {
        if (assets.length === 0) return;
        selectAsset(assets.filter((a) => a.media.state === "Ready")[0]);
    }, [assets]);

    return { selectedId, selectAsset, openAsset };
}

type AssetsGridProps = {
    assets: Asset[];
    onEndReached: () => void;
};

export function AssetsGrid({ assets, onEndReached }: AssetsGridProps) {
    const { colsCount, layoutReady, targetRef } = useGridLayout();
    const { refillTriggerRef } = useListRefill(onEndReached);
    const { selectedId, selectAsset, openAsset } = useAssetActions(assets);

    return (
        <div
            className="grid gap-0.5 relative"
            ref={targetRef}
            style={{
                gridTemplateColumns: `repeat(${colsCount}, minmax(0, 1fr))`,
            }}
        >
            {layoutReady &&
                assets
                    .filter((a) => a.media.state === "Ready")
                    .map((a) => (
                        <GridAsset
                            asset={a}
                            key={a.id}
                            selected={a.id === selectedId}
                            onSelect={selectAsset}
                            onOpen={openAsset}
                        />
                    ))}

            <div
                ref={refillTriggerRef}
                className="absolute h-125 md:h-100 w-full bottom-0 pointer-events-none"
            ></div>
        </div>
    );
}

export type GridAssetProps = {
    asset: Asset;
    selected: boolean;
    onSelect: (asset: Asset) => void;
    onOpen: (asset: Asset) => void;
};

function GridAsset({ asset, selected, onSelect, onOpen }: GridAssetProps) {
    return (
        <div
            data-selected={selected}
            className="
            group/grid-asset 
            cursor-pointer 
            bg-transparent hover:bg-foreground/4 data-[selected=true]:bg-foreground/8 
            rounded-md p-1 
            flex flex-col 
            outline-none"
            onClick={() => onSelect(asset)}
            onFocus={() => onSelect(asset)}
            onDoubleClick={() => onOpen(asset)}
            tabIndex={1}
        >
            <div className="aspect-square flex justify-center items-center p-1">
                <AssetMedia
                    className="
                    border border-border/75 overflow-hidden rounded-md select-none
                    group-data-[selected=true]/grid-asset:border-ring"
                    media={asset.media}
                />
            </div>
            <div className="w-full px-6">
                <p className="overflow-hidden text-ellipsis whitespace-nowrap text-[1.125rem] text-primary/90 text-center">
                    {asset.title}
                </p>
            </div>
        </div>
    );
}
