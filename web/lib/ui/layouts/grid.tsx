import type { Asset } from "@lib/api/types";

/**
 * Grid assets layout root
 */
function GridAssetsLayout({ assets }: { assets: Asset[] }) {
    return <h1>{assets[0] ? assets[0].title : ""}</h1>;
}

export { GridAssetsLayout };
