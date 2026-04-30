import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router";

import { useApi } from "@/api/context";

import type { Asset } from "@/features/project/assets/models";
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
        <div className="size-full grid pt-4">
            <div className="flex justify-center justify-self-center w-200">
                <div className="w-full border">
                    <AssetMedia media={asset.media} />
                </div>
                <div className="w-full">
                    <h2 className="text-xl w-full whitespace-normal wrap-anywhere font-medium">
                        {asset.title}
                    </h2>
                </div>
            </div>
        </div>
    );
}
