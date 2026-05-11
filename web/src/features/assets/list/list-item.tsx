import type { Assets } from "@/api/models";
import { useApi } from "@/providers";
import { useEffect, useState } from "react";

export function useAssetItem(listAsset: Assets.Asset) {
    const { assetsApiV1 } = useApi();
    const [asset, setAsset] = useState<Assets.Asset>(listAsset);

    async function assetPool() {
        try {
            const pooled = await assetsApiV1.get(asset.id);
            const status = pooled.media.status;

            if (status === "Ready" || status === "Failed") {
                setAsset(pooled);
                return;
            } else {
                setTimeout(assetPool, 1750);
            }
        } catch {
            setAsset((a) => ({
                ...a,
                media: {
                    ...a.media,
                    status: "Failed",
                },
            }));
        }
    }

    useEffect(() => {
        if (asset.media.status !== "Ready" && asset.media.status !== "Failed") {
            setTimeout(assetPool, 750);
        }
    }, []);

    return { asset, assetReady: asset.media.status === "Ready" };
}
