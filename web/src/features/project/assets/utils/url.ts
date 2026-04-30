import type { Asset } from "../models";

export function getAssetViewUrl(asset: Asset) {
    return `/asset/${asset.id}`;
}
