import type { Asset } from "@lib/api/types";

import { AssetInspector } from "../features/assets/asset-inspector";

export type InspectorView = {
    type: "display_asset";
    asset: Asset;
};

export function DisplayView({ view }: { view: InspectorView }) {
    switch (view.type) {
        case "display_asset":
            return <AssetInspector asset={view.asset} />;
    }
}
