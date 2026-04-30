import { useEffect, useState } from "react";

import { useApi } from "../api/context";

import { Scrollable } from "@/ui";
import { AssetsGrid } from "@/features/project/assets/ui";
import type { Asset } from "@/features/project/assets/models";

/**
 * Application Home Page
 */
function HomePage() {
    const [assets, setAssets] = useState<Asset[]>([]);

    const { assetApi } = useApi();

    useEffect(() => {
        (async () => {
            const assets = await assetApi.getList();
            setAssets(assets);
        })();
    }, []);

    return (
        <div className="w-full">
            <Scrollable className="h-screen w-full p-4">
                <AssetsGrid assets={assets} />
            </Scrollable>
        </div>
    );
}

export { HomePage };
