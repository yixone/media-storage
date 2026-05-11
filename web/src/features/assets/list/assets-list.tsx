import { useEffect, useRef } from "react";
import { useNavigate } from "react-router";

import { useAssetsList } from "@/providers";

import { AssetsListScroll } from "./assets-list-scroll";
import { AssetsGrid } from "../grid";
import type { Assets } from "@/api/models";

export function AssetsList() {
    const navigate = useNavigate();

    const { assets, nextBatch } = useAssetsList();
    const scrollRef = useRef<HTMLDivElement | null>(null);

    const listLayout = new AssetsGrid();

    function openAsset(asset: Assets.Asset) {
        navigate(`/asset/${asset.id}`);
    }

    useEffect(() => {
        (async () => {
            if (assets.length == 0) {
                await nextBatch();
            }
        })();
    }, []);

    return (
        <AssetsListScroll
            ref={scrollRef}
            onEndReached={nextBatch}
            useEndTrigger={true}
        >
            {listLayout.render(assets, {
                onOpenAsset: openAsset,
            })}
        </AssetsListScroll>
    );
}
