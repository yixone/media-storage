import { useEffect, useState } from "react";

import { useApi } from "@lib/api/context";
import type { Asset } from "@lib/api/types";

import { AssetsGrid } from "@lib/ui/components/features/assets";

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
        <div
            className="flex items-start justify-center p-4 w-full h-screen"
            style={{ scrollbarWidth: "thin" }}
        >
            <AssetsGrid assets={assets} />
        </div>
    );
}

export { HomePage };
