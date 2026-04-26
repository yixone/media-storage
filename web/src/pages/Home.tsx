import { useEffect, useState } from "react";

import { useApi } from "@lib/api/context";
import type { Asset } from "@lib/api/types";

import { AssetsGrid } from "@lib/ui/components/features/assets";
import { useInspector } from "@lib/ui/components/inspector";

/**
 * Application Home Page
 */
function HomePage() {
    const [assets, setAssets] = useState<Asset[]>([]);

    const { assetApi } = useApi();
    const { push } = useInspector();

    useEffect(() => {
        (async () => {
            const assets = await assetApi.getList();
            push({ type: "display_asset", asset: assets[0] });
            setAssets(assets);
        })();
    }, []);

    return (
        <div
            className="flex items-start justify-center p-4 w-full h-screen overflow-y-scroll"
            style={{ scrollbarWidth: "thin" }}
        >
            <AssetsGrid assets={assets} />
        </div>
    );
}

export { HomePage };
