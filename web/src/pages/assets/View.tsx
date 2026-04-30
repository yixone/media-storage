import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router";

import { useApi } from "@/api/context";

import type { Asset } from "@/features/project/assets/models";
import { AssetMedia } from "@/features/project/assets/ui";
import { getDisplaySize } from "@/features/project/media/displaySize";

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
        <div className="relative size-full flex flex-col md:flex-row h-screen">
            <div className="size-full flex justify-center items-center p-10 overflow-hidden bg-muted/65">
                {asset.media.state !== "Ready" && <h1>Processing...</h1>}
                {asset.media.state === "Ready" && (
                    <AssetMedia
                        className="border border-border/75 overflow-hidden rounded-xl"
                        media={asset.media}
                    />
                )}
            </div>
            <div className="h-full w-2/5 bg-background py-10 px-4 flex flex-col">
                <h2 className="text-3xl w-full whitespace-normal wrap-anywhere font-medium">
                    {asset.title}
                </h2>

                <h2 className="text-lg opacity-60">
                    {asset.media.mimetype} - {getDisplaySize(asset.media.size)}
                </h2>
            </div>
        </div>
    );
}
