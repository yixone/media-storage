import { createRoot } from "react-dom/client";

import "@lib/ui/style.css";
import { ApiProvider } from "@lib/api/context";
import { ApiClient } from "@lib/api/client";

import { HomePage } from "./pages";
import { BrowserRouter, Route, Routes } from "react-router";

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
                </Routes>
            </BrowserRouter>
        </ApiProvider>
    );
}

createRoot(document.getElementById("root")!).render(<Application />);
