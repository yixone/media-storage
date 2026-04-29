import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router";

import { useApi } from "@lib/api/context";
import type { Asset } from "@lib/api/types";

import { AssetMedia } from "../components/ui/asset/AssetMedia";

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
        <div className="size-full pt-4">
            <AssetMedia className="border max-w-100" media={asset.media} />
        </div>
    );
}
