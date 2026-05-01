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

import { ArrowLeftIcon } from "@/ui/icons";
import { Button } from "@/ui";

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

    const navigate = useNavigate();

    if (!asset) return null;

    return (
        <AssetViewLayout className="w-full h-screen">
            <AssetViewContent className="p-0 md:p-4 flex items-center justify-center relative">
                <div className="absolute top-0 left-0 p-2 md:p-4 z-4">
                    <Button
                        variant="outline"
                        className="size-11 p-1.5"
                        size="icon"
                        onClick={() => {
                            navigate(-1);
                        }}
                    >
                        <ArrowLeftIcon className="text-foreground w-full" />
                    </Button>
                </div>
                <AssetViewMediaContainer
                    aspectRatio={getAspectRatio(asset.media)}
                >
                    <div className="">
                        <AssetMedia
                            className="size-full md:border border-border/75 overflow-hidden md:rounded-md"
                            media={asset.media}
                        />
                    </div>
                </AssetViewMediaContainer>
            </AssetViewContent>
            <AssetViewDetails className="p-4">
                <h2 className="text-3xl md:text-2xl w-full whitespace-normal wrap-anywhere font-medium">
                    {asset.title}
                </h2>

                <h2 className="opacity-60 text-lg">
                    {asset.media.mimetype} - {getDisplaySize(asset.media.size)}
                </h2>
            </AssetViewDetails>
        </AssetViewLayout>
    );
}
