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
import { getAspectRatio } from "@/features/project/media/utils";
import { AssetMedia } from "@/features/project/assets/ui";
import { getDisplaySize } from "@/features/project/media/utils/displaySize";

function useTarget(id?: string) {
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

    return { asset };
}

export function AssetViewPage() {
    const { id } = useParams();
    const { asset } = useTarget(id);

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
                <AssetDetails asset={asset} />
            </AssetViewDetails>
        </AssetViewLayout>
    );
}

type AssetDetailsProps = { asset: Asset };

function AssetDetails({ asset }: AssetDetailsProps) {
    return (
        <div>
            <h2 className="text-2xl w-full whitespace-normal wrap-anywhere font-medium">
                {asset.title}
            </h2>

            <h2 className="opacity-60 text-lg">
                {asset.media.mimetype} - {getDisplaySize(asset.media.size)}
            </h2>
        </div>
    );
}
