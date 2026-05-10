import type { Assets } from "@/api/models";

export type AssetsListEvents = {
    onOpenAsset: (asset: Assets.Asset) => void;
};

export interface AssetsListLayout {
    render(list: Assets.Asset[], events: AssetsListEvents): React.ReactNode;
}
