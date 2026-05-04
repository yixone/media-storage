import { useEffect, useRef, useState } from "react";

import type { Assets } from "@/api/models";
import { useApi, useInspector } from "@/providers";
import { AssetsListScroll } from "./assets-list-scroll";
import { AssetInspector } from "@/features/project/assets/ui";
import { AssetsGrid } from "../grid";
import { useNavigate } from "react-router";

const PAGINATION_LIMIT = 25;

function useAssetsListData() {
    const { assetsApiV0 } = useApi();

    const [assets, setAssets] = useState<Assets.Asset[]>([]);

    const assetsLoading = useRef(false);
    const assetsOut = useRef(false);

    const listOffset = useRef(0);

    async function loadAssets(offset: number, limit: number) {
        const invalidPagination = offset < 0 || limit <= 0;
        if (assetsLoading.current || assetsOut.current || invalidPagination) {
            return;
        }

        assetsLoading.current = true;

        const data = await assetsApiV0.getList(offset, limit);
        setAssets((a) => [...a, ...data]);

        if (data.length < limit) {
            assetsOut.current = true;
        }

        assetsLoading.current = false;
    }

    async function loadMoreAssets() {
        await loadAssets(listOffset.current, PAGINATION_LIMIT);
        listOffset.current += PAGINATION_LIMIT;
    }

    useEffect(() => {
        (async () => {
            if (assets.length > 0) return;
            await loadAssets(0, PAGINATION_LIMIT);
            listOffset.current += PAGINATION_LIMIT;
        })();
    }, []);

    return { assets, loadMoreAssets, assetsOut };
}

function useAssetSelection() {
    const { addView } = useInspector();

    const [selectedId, setSelectedId] = useState<string | null>(null);

    function selectAsset(asset: Assets.Asset) {
        setSelectedId(asset.id);
        addView(new AssetInspector(asset));
    }

    return { selectedId, selectAsset };
}

function useAssetNavigation() {
    const navigate = useNavigate();

    function openAsset(asset: Assets.Asset) {
        navigate(`/asset/${asset.id}`);
    }

    return { openAsset };
}

export function AssetsList() {
    const { assets, loadMoreAssets, assetsOut } = useAssetsListData();

    const { selectedId, selectAsset } = useAssetSelection();
    const { openAsset } = useAssetNavigation();

    useEffect(() => {
        if (assets.length > 0 && selectedId === null) {
            selectAsset(assets.filter((a) => a.media.state === "Ready")[0]);
        }
    }, [assets]);

    const listLayout = new AssetsGrid();

    return (
        <AssetsListScroll
            onEndReached={loadMoreAssets}
            useEndTrigger={!assetsOut.current}
        >
            {listLayout.render(assets, selectedId, {
                onSelectAsset: selectAsset,
                onOpenAsset: openAsset,
            })}
        </AssetsListScroll>
    );
}
