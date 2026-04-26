import { useLocation } from "react-router";
import { useInspector } from "./Provider";

const INSPECTOR_WIDTH = "30rem";

export function Inspector() {
    const location = useLocation();

    // FIXME: implement a better solution
    if (location.pathname === "/upload") return;

    const { node } = useInspector();

    return (
        <div
            className="
                bg-card 
                border-l border-border
                h-screen
                overflow-hidden
                "
            style={{ width: INSPECTOR_WIDTH }}
        >
            {node}
        </div>
    );
}
