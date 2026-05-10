import { useEffect, useRef, useState } from "react";

import type { Assets } from "@/api/models";
import { useApi } from "@/providers";

import { AssetsListScroll } from "./assets-list-scroll";
import { AssetsGrid } from "../grid";

const PAGINATION_LIMIT = 25;

function useAssetsListData() {
    const { assetsApiV1 } = useApi();

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

        const data = await assetsApiV1.getList(offset, limit);
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

export function AssetsList() {
    const { assets, loadMoreAssets, assetsOut } = useAssetsListData();

    const listLayout = new AssetsGrid();

    return (
        <AssetsListScroll
            onEndReached={loadMoreAssets}
            useEndTrigger={!assetsOut.current}
        >
            {listLayout.render(assets)}
        </AssetsListScroll>
    );
}
