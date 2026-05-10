import type { Assets } from "@/api/models";
import { AssetMedia } from "../asset-media";

type AssetsGridItemProps = {
    asset: Assets.Asset;
};

export function AssetsGridItem({ asset }: AssetsGridItemProps) {
    return (
        <a
            className="
                cursor-pointer 
                bg-transparent hover:bg-foreground/4 data-[selected=true]:bg-foreground/8 
                rounded-md p-1 
                flex flex-col 
                outline-none"
            draggable={false}
            href={`asset/${asset.id}`}
        >
            <div className="aspect-square flex justify-center items-center p-1">
                <AssetMedia
                    className="
                        border border-border/75 overflow-hidden rounded-md select-none
                        group-data-[selected=true]/grid-asset:border-ring"
                    media={asset.media}
                />
            </div>
            <div className="w-full px-6">
                <p className="overflow-hidden text-ellipsis whitespace-nowrap text-[1.125rem] text-primary/90 text-center">
                    {asset.title}
                </p>
            </div>
        </a>
    );
}
