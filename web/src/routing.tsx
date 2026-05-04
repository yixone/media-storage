import { Route, Routes } from "react-router";

import { BrowseLayout, MinimalLayout } from "@/layouts";

import { HomePage } from "@/pages";
import { AssetCreatePage, AssetViewPage } from "@/pages/asset";

export function AppRoutes() {
    return (
        <Routes>
            <Route element={<BrowseLayout />}>
                <Route index element={<HomePage />} />
            </Route>

            <Route element={<MinimalLayout />}>
                <Route path="/asset/create" element={<AssetCreatePage />} />
                <Route path="/asset/:id" element={<AssetViewPage />} />
            </Route>
        </Routes>
    );
}
