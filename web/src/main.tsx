import { createRoot } from "react-dom/client";
import { BrowserRouter } from "react-router";

import { ApiClient } from "@/api";

import "./style.css";
import { AppRoutes } from "./routing";
import { ApiProvider, AssetsListProvider } from "./providers";

/**
 * Configures application
 */
function Application() {
    const client = new ApiClient("http://localhost:8080");

    return (
        <ApiProvider client={client}>
            <AssetsListProvider listPageSize={15}>
                <BrowserRouter>
                    <AppRoutes />
                </BrowserRouter>
            </AssetsListProvider>
        </ApiProvider>
    );
}

const root = createRoot(document.getElementById("root")!);
root.render(<Application />);
