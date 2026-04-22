import { createRoot } from "react-dom/client";

import "./index.css";
import { HomePage } from "./pages";
import { ApiClient } from "@lib/api/client";
import { ApiProvider } from "@lib/api/context";

/**
 * Configures application
 */
function Application() {
    const client = new ApiClient("localhost:8080");

    return (
        <ApiProvider client={client}>
            <HomePage />
        </ApiProvider>
    );
}

createRoot(document.getElementById("root")!).render(<Application />);
