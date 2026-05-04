import type { Assets } from "@/api/models";
import type { InspectorView } from "../inspector";
import { AssetMedia } from "./asset-media";
import { Separator } from "../shared/ui";
import { humanMediaSize } from "../media";

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

                <div className="flex flex-col gap-1 px-2">
                    <Separator />
                    <h2 className="text-xl w-full whitespace-normal wrap-anywhere font-medium">
                        {this.asset.title}
                    </h2>
                </div>
            </div>
        );
    }
}
