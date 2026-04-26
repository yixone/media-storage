import { useInspector } from "./provider";
import { DisplayView } from "./view";

const INSPECTOR_WIDTH = "30rem";

export function Inspector() {
    const { stack } = useInspector();
    const current = stack[stack.length - 1];

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
            {current && <DisplayView view={current} />}
        </div>
    );
}
