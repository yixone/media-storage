import { createRoot } from "react-dom/client";
import { BrowserRouter } from "react-router";

import { ApiClient } from "@/api";

import "./style.css";
import { AppRoutes } from "./routing";
import { ApiProvider } from "./providers";

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

const root = createRoot(document.getElementById("root")!);
root.render(<Application />);
