import { createRoot } from "react-dom/client";
import { BrowserRouter } from "react-router";

import { ApiClient, ApiProvider } from "@/api";

import "./style.css";
import { AppRoutes } from "./routing";

/**
 * Configures application
 */
function Application() {
    const client = new ApiClient("http://localhost:8080");

    return (
        <ApiProvider client={client}>
            <BrowserRouter>
                <AppRoutes />
            </BrowserRouter>
        </ApiProvider>
    );
}

createRoot(document.getElementById("root")!).render(<Application />);
