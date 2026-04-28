import { useInspector } from "../../providers";
import { AssetInspector } from "./assets";

const INSPECTOR_WIDTH = "25rem";

export function Inspector() {
    const { activeView } = useInspector();

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
            {activeView?.type == "display.asset" && (
                <AssetInspector asset={activeView.asset} />
            )}
        </div>
    );
}
