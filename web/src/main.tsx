import { createRoot } from "react-dom/client";

import "@lib/ui/style.css";
import { ApiProvider } from "@lib/api/context";
import { ApiClient } from "@lib/api/client";

import { HomePage } from "./pages";

/**
 * Configures application
 */
function Application() {
    const client = new ApiClient("http://localhost:8080");

    return (
        <ApiProvider client={client}>
            <HomePage />
        </ApiProvider>
    );
}

createRoot(document.getElementById("root")!).render(<Application />);
