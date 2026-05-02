import type { InspectorView } from "@/features/inspector";
import type { Assets } from "@/api/models";
import { humanMediaSize } from "@/features/media";
import { AssetMedia } from "@/features/assets";

export class AssetInspector implements InspectorView {
    viewType = "display.asset";
    asset: Assets.Asset;

    public constructor(asset: Assets.Asset) {
        this.asset = asset;
    }

    public render(): React.ReactNode {
        return (
            <div className="grid p-2 pt-4 gap-3">
                <div className="aspect-square flex justify-center items-center">
                    <AssetMedia
                        media={this.asset.media}
                        className="border border-border/90 rounded-md overflow-hidden"
                    />
                </div>

                <div className="flex flex-col gap-0.5 px-2">
                    <h2 className="text-xl w-full whitespace-normal wrap-anywhere font-medium">
                        {this.asset.title}
                    </h2>

                    <h2 className="opacity-60">
                        {`${this.asset.media.mimetype} - ${humanMediaSize(
                            this.asset.media.size
                        )}`}
                    </h2>
                </div>
            </div>
        );
    }
}

// export function AssetInspector({ asset }: { asset: Asset }) {
//     return (
//         <div className="grid p-2 pt-4 gap-3">
//             <div className="aspect-square flex justify-center items-center">
//                 <AssetMedia
//                     media={asset.media}
//                     className="border border-border/90 rounded-md overflow-hidden"
//                 />
//             </div>

//             <div className="flex flex-col gap-0.5 px-2">
//                 <h2 className="text-xl w-full whitespace-normal wrap-anywhere font-medium">
//                     {asset.title}
//                 </h2>

//                 <h2 className="opacity-60">
//                     {asset.media.mimetype} - {getDisplaySize(asset.media.size)}
//                 </h2>
//             </div>
//         </div>
//     );
// }
