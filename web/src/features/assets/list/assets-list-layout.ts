import type { Assets } from "@/api/models";

export interface AssetsListLayout {
    render(list: Assets.Asset[]): React.ReactNode;
}
