import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router";

import {
    AssetViewContent,
    AssetViewDetails,
    AssetViewLayout,
    AssetViewMediaContainer,
} from "./AssetView.layout";

import type { Assets } from "@/api/models";

import { AssetMedia } from "@/features/assets";
import { mediaAspectRatio, humanMediaSize } from "@/features/media";
import { ArrowLeftIcon } from "@/features/shared/ui/icons";
import { Button } from "@/features/shared/ui";

import { useApi } from "@/providers";

function useTarget(id?: string) {
    const { assetsApiV0 } = useApi();

    const [asset, setAsset] = useState<Assets.Asset | null>(null);
    const navigate = useNavigate();

    useEffect(() => {
        (async () => {
            if (!id) return;

            try {
                const asset = await assetsApiV0.get(id);
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
                        size="icon"
                        className="size-10 md:size-11 p-1 md:p-1.5"
                        onClick={() => {
                            navigate(-1);
                        }}
                    >
                        <ArrowLeftIcon className="text-foreground w-full" />
                    </Button>
                </div>
                <AssetViewMediaContainer
                    aspectRatio={mediaAspectRatio(asset.media)}
                >
                    <div className="box-border">
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
                    {asset.media.mimetype} - {humanMediaSize(asset.media.size)}
                </h2>
            </AssetViewDetails>
        </AssetViewLayout>
    );
}
