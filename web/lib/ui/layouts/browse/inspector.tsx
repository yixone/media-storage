import { useInspector } from "./providers";

const INSPECTOR_WIDTH = "30rem";

export function Inspector() {
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
