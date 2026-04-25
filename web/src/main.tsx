import { createRoot } from "react-dom/client";

import { BrowserRouter, Route, Routes } from "react-router";

import { ApiProvider } from "@lib/api/context";
import { ApiClient } from "@lib/api/client";

import "@lib/ui/style";
import { HomePage } from "./pages/Home";
import { UploadPage } from "./pages/Upload";

/**
 * Configures application
 */
function Application() {
    const client = new ApiClient("http://localhost:8080");

    return (
        <ApiProvider client={client}>
            <BrowserRouter>
                <Routes>
                    <Route index element={<HomePage />} />

                    <Route path="/upload" element={<UploadPage />} />
                </Routes>
            </BrowserRouter>
        </ApiProvider>
    );
}

createRoot(document.getElementById("root")!).render(<Application />);
