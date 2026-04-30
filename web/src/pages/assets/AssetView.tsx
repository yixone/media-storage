import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router";

import { useApi } from "@/api/context";

import type { Asset } from "@/features/project/assets/models";
import {
    AssetViewContent,
    AssetViewDetails,
    AssetViewLayout,
    AssetViewMediaContainer,
} from "./AssetView.layout";
import { getAspectRatio } from "@/features/project/media/aspectRatio";
import { AssetMedia } from "@/features/project/assets/ui";

export function AssetViewPage() {
    const { id } = useParams();
    const { assetApi } = useApi();

    const [asset, setAsset] = useState<Asset | null>(null);
    const navigate = useNavigate();

    useEffect(() => {
        (async () => {
            if (!id) return;

            try {
                const asset = await assetApi.get(id);
                setAsset(asset);
            } catch (err) {
                navigate("/");
            }
        })();
    }, []);

    if (!asset) return null;

    return (
        <AssetViewLayout className="w-full h-screen">
            <AssetViewContent className="p-1 md:p-4">
                <AssetViewMediaContainer
                    aspectRatio={getAspectRatio(asset.media)}
                >
                    <AssetMedia
                        className="border border-border/75 overflow-hidden rounded-xl md:rounded-md"
                        media={asset.media}
                    />
                </AssetViewMediaContainer>
            </AssetViewContent>
            <AssetViewDetails className="p-4">
                <div className="flex flex-col">
                    <h2 className="text-xl w-full whitespace-normal wrap-anywhere font-medium">
                        {asset.title}
                    </h2>
                </div>
            </AssetViewDetails>
        </AssetViewLayout>
    );
}
