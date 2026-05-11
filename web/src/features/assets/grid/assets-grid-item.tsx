import type { Assets } from "@/api/models";
import { useAssetItem } from "../list";
import { Skeleton } from "@/features/shared/ui";
import { cn } from "@/utils";
import { MediaContainer, MediaImage } from "@/features/media";

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
                    <MediaContainer
                        className="border border-border/75 overflow-hidden rounded-md select-none"
                        media={asset.media}
                    >
                        <MediaImage
                            loading="lazy"
                            mediaId={asset.media.id}
                            format="preview"
                        />
                    </MediaContainer>
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
