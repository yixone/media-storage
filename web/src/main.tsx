import { createRoot } from "react-dom/client"

import "./index.css"
import { HomePage } from "./pages"

/**
 * Configures application
 */
function Application() {
	return <HomePage />
}

createRoot(document.getElementById("root")!).render(<Application />)
