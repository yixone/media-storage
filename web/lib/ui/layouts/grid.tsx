import { useApi } from "@lib/api/context";
import type { Asset, Media } from "@lib/api/types";
import { useState } from "react";

/**
 * Assets grid layout
 */
function AssetsGridLayout({ assets }: { assets: Asset[] }) {
    return (
        <div
            className="
            grid grid-cols-7
            p-2 gap-2
            overflow-hidden
            "
        >
            {assets.map((a) => (
                <GridAsset asset={a} key={a.id}>
                    <GridAssetMedia media={a.media} />
                    <GridAssetData title={a.title} />
                </GridAsset>
            ))}
        </div>
    );
}

/**
 * Container for the grid layout asset
 */
function GridAsset({
    asset,
    children,
}: React.ComponentProps<"div"> & { asset: Asset }) {
    return (
        <div className="block">
            <a
                className="hover:*:brightness-95 *:transition-[filter] *:duration-75"
                href={`/a/${asset.id}`}
            >
                {children}
            </a>
        </div>
    );
}

/**
 * Media component for the grid layout asset
 */
function GridAssetMedia({ media }: { media: Media }) {
    const { mediaApi } = useApi();
    const [loaded, setLoaded] = useState(false);

    return (
        <div
            className="
                    box-border aspect-square
                    relative
                    overflow-hidden
                    border border-black/15
                    "
        >
            {!loaded && (
                <div
                    className="
                            size-full
                            absolute
                            "
                    style={{ backgroundColor: `#${media.color}` }}
                />
            )}
            <img
                className="
                        size-full
                        object-cover
                        transition-[filter] duration-75
                        "
                src={mediaApi.getMediaUrl(media.id)}
                onLoad={() => {
                    setLoaded(true);
                }}
                style={{
                    visibility: loaded ? "visible" : "hidden",
                    filter: loaded ? "none" : "blur(25px)",
                }}
            />
        </div>
    );
}

/**
 * A component for displaying metadata for a grid layout asset
 */
function GridAssetData({ title }: { title: string | null }) {
    return <p className="overflow-hidden text-ellipsis">{title}</p>;
}

export { AssetsGridLayout };
