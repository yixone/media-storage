import type { Asset } from "@lib/api/types";

import { DateDisplay } from "../date";
import { MediaDisplay } from "../media";

export function AssetInspector({ asset }: { asset: Asset }) {
    const aspectRatio = (asset.media.width ?? 1) / (asset.media.height ?? 1);
    return (
        <div
            className="
            flex flex-col
            w-full
            p-4
            gap-2
            "
        >
            <div
                className="w-full max-h-100 min-h-5"
                style={{
                    aspectRatio,
                }}
            >
                <div className="flex size-full justify-center">
                    <div>
                        <MediaDisplay
                            media={asset.media}
                            className="overflow-hidden border border-border/65 rounded-[0.5rem] max-h-100 min-h-5 size-full"
                        />
                    </div>
                </div>
            </div>

            <h2 className="text-xl w-full whitespace-normal wrap-anywhere">
                {asset.title}
            </h2>
            {asset.source_url && (
                <a href={asset.source_url} className="text-foreground/65">
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
