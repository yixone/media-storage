import { getDisplaySize } from "../../media/displaySize";
import type { Asset } from "../models";
import { AssetMedia } from "./AssetMedia";

export function AssetInspector({ asset }: { asset: Asset }) {
    return (
        <div className="grid p-4 gap-3">
            <div className="aspect-square flex justify-center items-center">
                <AssetMedia
                    media={asset.media}
                    className="border border-border/90 rounded-md overflow-hidden"
                />
            </div>

            <div className="flex flex-col gap-0.5">
                <h2 className="text-xl w-full whitespace-normal wrap-anywhere font-medium">
                    {asset.title}
                </h2>

                <h2 className="opacity-60">
                    {asset.media.mimetype} - {getDisplaySize(asset.media.size)}
                </h2>
            </div>
        </div>
    );
}
