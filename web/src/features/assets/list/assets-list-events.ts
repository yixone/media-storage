import { useNavigate } from "react-router";

import type { Assets } from "@/api/models";

export function useAssetNavigation() {
    const navigate = useNavigate();

    function openAsset(asset: Assets.Asset) {
        navigate(`/asset/${asset.id}`);
    }

    return { openAsset };
}
