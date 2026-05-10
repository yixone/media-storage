import type { Assets } from "@/api/models";
import { AssetMedia } from "../asset-media";
import { useAssetItem } from "../list";
import { Skeleton } from "@/features/shared/ui";
import { cn } from "@/utils";

type AssetsGridItemProps = {
    listAsset: Assets.Asset;

    onOpen: (asset: Assets.Asset) => void;
};

export function AssetsGridItem({ listAsset, onOpen }: AssetsGridItemProps) {
    const { asset, assetReady } = useAssetItem(listAsset);

    return (
        <div
            className={cn(
                "bg-transparent rounded-md p-1 flex flex-col outline-none",
                assetReady
                    ? "cursor-pointer hover:bg-foreground/4"
                    : "cursor-default"
            )}
            onClick={() => {
                if (assetReady) {
                    onOpen(asset);
                }
            }}
            tabIndex={assetReady ? 1 : -1}
        >
            <div className="aspect-square flex justify-center items-center p-1">
                {assetReady && (
                    <AssetMedia
                        className="
                        border border-border/75 overflow-hidden rounded-md select-none
                        group-data-[selected=true]/grid-asset:border-ring"
                        media={asset.media}
                    />
                )}
                {!assetReady && <Skeleton className="size-full" />}
            </div>
            <div className="w-full px-6">
                <p className="overflow-hidden text-ellipsis whitespace-nowrap text-[1.125rem] text-primary/90 text-center">
                    {asset.title}
                </p>
            </div>
        </div>
    );
}
