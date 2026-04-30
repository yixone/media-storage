import { useEffect, useState } from "react";
import { useNavigate } from "react-router";

import { useInspector } from "@/features/common/inspector";

import { AssetMedia } from "./AssetMedia";
import type { Asset } from "../models";
import { getAssetViewUrl } from "../utils/url";
import {
    AssetsGridLayout,
    GridAssetLayout,
    GridAssetMediaLayout,
} from "./AssetsGrid.layout";
import { Skeleton } from "@/ui";

function AssetsGrid({ assets }: { assets: Asset[] }) {
    const [selectedId, setSelectedId] = useState<string | null>(null);
    const { addView } = useInspector();

    const selectAsset = (asset: Asset) => {
        setSelectedId(asset.id);
        addView({ type: "display.asset", asset });
    };

    useEffect(() => {
        if (assets.length === 0) return;
        selectAsset(assets[0]);
    }, [assets]);

    return (
        <AssetsGridLayout>
            {assets.map((a) => (
                <GridAsset
                    asset={a}
                    key={a.id}
                    selected={a.id === selectedId}
                    onSelect={selectAsset}
                />
            ))}
        </AssetsGridLayout>
    );
}

export type GridAssetProps = {
    asset: Asset;

    selected: boolean;
    onSelect: (asset: Asset) => void;
};

export type GridAssetPlaceholderProps = {
    asset: Asset;
};

function GridAsset({ asset, selected, onSelect }: GridAssetProps) {
    if (asset.media.state !== "Ready")
        return <GridAssetPlaceholder asset={asset} />;

    const navigate = useNavigate();

    const goToAssetPage = () => {
        navigate(getAssetViewUrl(asset));
    };

    return (
        <GridAssetLayout
            data-selected={selected}
            onClick={() => onSelect(asset)}
            onFocus={() => onSelect(asset)}
            onDoubleClick={() => goToAssetPage()}
            tabIndex={1}
        >
            <GridAssetMediaLayout className="p-1">
                <AssetMedia
                    className="
                    border border-border/75 overflow-hidden rounded-md select-none 
                    group-data-[selected=true]/grid-asset:border-ring
                    group-data-[selected=true]/grid-asset:outline
                    group-data-[selected=true]/grid-asset:outline-ring
                    "
                    media={asset.media}
                />
            </GridAssetMediaLayout>
            <GridAssetMeta title={asset.title} />
        </GridAssetLayout>
    );
}

function GridAssetPlaceholder({ asset }: GridAssetPlaceholderProps) {
    return (
        <GridAssetLayout>
            <GridAssetMediaLayout className="p-2">
                <Skeleton className="size-full" />
            </GridAssetMediaLayout>
            <GridAssetMeta title={asset.title} />
        </GridAssetLayout>
    );
}

export type GridAssetMetaProps = {
    title: string | null;
};

function GridAssetMeta({ title }: GridAssetMetaProps) {
    return (
        <div className="w-full px-8">
            <p className="overflow-hidden text-ellipsis whitespace-nowrap text-[1.125rem] text-primary/90 text-center">
                {title}
            </p>
        </div>
    );
}

export { AssetsGrid };
