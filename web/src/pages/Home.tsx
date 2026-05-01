import { useEffect, useRef, useState } from "react";

import { useApi } from "../api/context";

import { Scrollable } from "@/ui";
import { AssetsGrid } from "@/features/project/assets/ui";
import type { Asset } from "@/features/project/assets/models";

const PAGINATION_LIMIT = 30;

const useAssetsList = () => {
    const [assets, setAssets] = useState<Asset[]>([]);

    const assetsLoading = useRef(false);
    const listCursor = useRef(0);

    const noMoreAssets = useRef(false);

    const { assetApi } = useApi();

    const callRefill = async () => {
        if (assetsLoading.current || noMoreAssets.current) return;
        console.log("[ASSETS REFILLER] Requesting additional assets");

        assetsLoading.current = true;
        listCursor.current += PAGINATION_LIMIT;

        const refill = await assetApi.getList(
            listCursor.current,
            PAGINATION_LIMIT
        );
        setAssets((a) => [...a, ...refill]);

        if (refill.length < PAGINATION_LIMIT) {
            console.log("[ASSETS REFILLER] The storage is out of assets");
            noMoreAssets.current = true;
        }

        assetsLoading.current = false;
    };

    useEffect(() => {
        (async () => {
            if (assets.length > 0) return;

            assetsLoading.current = true;
            console.log("[ASSETS LIST] Requesting a list of assets");

            const assetsList = await assetApi.getList(0, PAGINATION_LIMIT);
            setAssets(assetsList);

            if (assetsList.length < PAGINATION_LIMIT) {
                noMoreAssets.current = true;
            }

            listCursor.current = 0;
            assetsLoading.current = false;
        })();
    }, []);

    return { assets, callRefill };
};

function HomePage() {
    const { assets, callRefill } = useAssetsList();

    return (
        <Scrollable className="h-screen w-full p-4">
            <AssetsGrid assets={assets} onEndReached={callRefill} />
        </Scrollable>
    );
}

export { HomePage };
