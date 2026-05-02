import { useInspector } from "@/providers";

const INSPECTOR_WIDTH = "25rem";

export function Inspector() {
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
            <InspectorOutlet />
        </div>
    );
}

export function InspectorOutlet() {
    const { activeView } = useInspector();

    if (!activeView) return null;

    return activeView.render();
}
