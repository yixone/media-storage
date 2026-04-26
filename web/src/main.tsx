import { createRoot } from "react-dom/client";

import { BrowserRouter, Route, Routes } from "react-router";

import { ApiProvider } from "@lib/api/context";
import { ApiClient } from "@lib/api/client";

import "@lib/ui/style";
import { Inspector, InspectorProvider } from "@lib/ui/components/inspector";

import { HomePage } from "./pages/Home";
import { UploadPage } from "./pages/Upload";
import { AppLayout } from "@lib/ui/components/layout/Base";

function Layout() {
    return (
        <AppLayout>
            <InspectorProvider>
                <Routes>
                    <Route index element={<HomePage />} />
                    <Route path="/upload" element={<UploadPage />} />
                </Routes>

                <Inspector />
            </InspectorProvider>
        </AppLayout>
    );
}

/**
 * Configures application
 */
function Application() {
    const client = new ApiClient("http://localhost:8080");

    return (
        <ApiProvider client={client}>
            <BrowserRouter>
                <Layout />
            </BrowserRouter>
        </ApiProvider>
    );
}

createRoot(document.getElementById("root")!).render(<Application />);
