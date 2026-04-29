import { createRoot } from "react-dom/client";

import { BrowserRouter, Route, Routes } from "react-router";

import { ApiProvider } from "@lib/api/context";
import { ApiClient } from "@lib/api/client";

import "./style";

import { HomePage } from "./pages/Home";
import { UploadPage } from "./pages/Upload";
import { BrowseLayout, ViewLayout } from "./layouts";
import { AssetViewPage } from "./pages/AssetView";

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
                        <Route path="/upload" element={<UploadPage />} />
                        <Route path="/a/:id" element={<AssetViewPage />} />
                    </Route>
                </Routes>
            </BrowserRouter>
        </ApiProvider>
    );
}

createRoot(document.getElementById("root")!).render(<Application />);
