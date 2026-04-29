import type { Asset } from "@lib/api/types";

import { DateDisplay } from "../Date";
import { AssetMedia } from "./AssetMedia";

export function AssetInspector({ asset }: { asset: Asset }) {
    return (
        <div className="flex flex-col p-4 gap-2">
            <AssetMedia className="max-h-100" media={asset.media} />

            <h2 className="text-xl w-full whitespace-normal wrap-anywhere font-medium">
                {asset.title}
            </h2>

            {asset.source_url && (
                <a href={asset.source_url!} className="text-foreground/65">
                    <b>{"Source: "}</b>
                    <i className="decoration-1 underline">{asset.source_url}</i>
                </a>
            )}

            <h2 className="text-foreground/65">
                <b>{"Created: "}</b>
                <DateDisplay date={new Date(asset.created_at)} />
            </h2>
            <h2 className="text-foreground/65">
                <b>{"Size: "}</b>
                {(asset.media.size / 1024 / 1024).toFixed(2)} Mb
            </h2>
        </div>
    );
}
