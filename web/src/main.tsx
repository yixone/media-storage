import { createRoot } from "react-dom/client";

import { BrowserRouter, Route, Routes } from "react-router";

import { ApiProvider } from "@/api/context";
import { ApiClient } from "@/api/client";

import "./style";

import { BrowseLayout, ViewLayout } from "@/layouts";

import { HomePage } from "@/pages/Home";
import { AssetViewPage, AssetCreatePage } from "@/pages/assets";

/**
 * Configures application
 */
function Application() {
    const client = new ApiClient("http://localhost:8080");

    return (
        <ApiProvider client={client}>
            <BrowserRouter>
                <Routes>
                    <Route element={<BrowseLayout />}>
                        <Route index element={<HomePage />} />
                    </Route>

                    <Route element={<ViewLayout />}>
                        <Route
                            path="/asset/create"
                            element={<AssetCreatePage />}
                        />
                        <Route path="/asset/:id" element={<AssetViewPage />} />
                    </Route>
                </Routes>
            </BrowserRouter>
        </ApiProvider>
    );
}

createRoot(document.getElementById("root")!).render(<Application />);
