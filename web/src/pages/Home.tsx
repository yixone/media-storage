import { useApi } from "@lib/api/context";
import type { Asset } from "@lib/api/types";
import { GridAssetsLayout } from "@lib/ui/layouts";
import { useEffect, useState } from "react";

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
        <div>
            <GridAssetsLayout assets={assets} />
        </div>
    );
}

export { HomePage };
