import { useState } from "react";
import { useNavigate } from "react-router";

import { useInspector } from "@/providers";
import type { Assets } from "@/api/models";
import { AssetInspector } from "../asset-inspector";

export function useAssetSelection() {
    const { addView } = useInspector();

    const [selectedId, setSelectedId] = useState<string | null>(null);

    function selectAsset(asset: Assets.Asset) {
        setSelectedId(asset.id);
        addView(new AssetInspector(asset));
    }

    return { selectedId, selectAsset };
}

export function useAssetNavigation() {
    const navigate = useNavigate();

    function openAsset(asset: Assets.Asset) {
        navigate(`/asset/${asset.id}`);
    }

    return { openAsset };
}
