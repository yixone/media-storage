import { useEffect, useState } from "react";

import { useApi } from "@lib/api/context";
import type { Asset } from "@lib/api/types";

import { AssetsGrid } from "../components/ui/asset";
import { Scrollable } from "../components/ui/base";

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
