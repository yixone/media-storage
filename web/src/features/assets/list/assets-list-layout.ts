import type { Assets } from "@/api/models";

export type AssetsListEvents = {
    onSelectAsset: (asset: Assets.Asset) => void;
    onOpenAsset: (asset: Assets.Asset) => void;
};

export interface AssetsListLayout {
    render(
        list: Assets.Asset[],
        selectedId: string | null,
        events: AssetsListEvents
    ): React.ReactNode;
}
