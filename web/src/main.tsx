import { createRoot } from "react-dom/client";

import "@lib/ui/style.css";
import { ApiProvider } from "@lib/api/context";
import { ApiClient } from "@lib/api/client";

import { HomePage } from "./pages";
import {
    AssetInspector,
    Inspector,
    InspectorProvider,
} from "@lib/ui/components/inspector";

/**
 * Configures application
 */
function Application() {
    const client = new ApiClient("http://localhost:8080");

    return (
        <ApiProvider client={client}>
            <div className="flex">
                <InspectorProvider>
                    <HomePage />
                    <Inspector>
                        <AssetInspector />
                    </Inspector>
                </InspectorProvider>
            </div>
        </ApiProvider>
    );
}

createRoot(document.getElementById("root")!).render(<Application />);
