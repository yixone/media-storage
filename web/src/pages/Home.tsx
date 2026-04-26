import { useEffect, useState } from "react";

import { useApi } from "@lib/api/context";
import type { Asset } from "@lib/api/types";

import { AssetsGrid } from "@lib/ui/components/features/assets";
import { PageLayout } from "@lib/ui/components/layout/Page";

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
        <PageLayout variant="scrollable" className="p-4 w-full h-screen">
            <AssetsGrid assets={assets} />
        </PageLayout>
    );
}

export { HomePage };
