import { useApi } from "@lib/api/context";
import type { Asset } from "@lib/api/types";
import { AssetsGridLayout } from "@lib/ui/layouts";
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
        <div
            className="
            p-4 
            w-full h-screen
            overflow-y-scroll
            "
            style={{ scrollbarWidth: "thin" }}
        >
            <AssetsGridLayout assets={assets} />
        </div>
    );
}

export { HomePage };
