import { Resizable } from "@lib/ui/components/base";
import { useInspector } from "../../providers";
import { AssetInspector } from "./assets";

const INSPECTOR_WIDTH = 25;

export function Inspector() {
    const { activeView } = useInspector();

    return (
        <Resizable defaultWidth={INSPECTOR_WIDTH}>
            <div
                className="
                bg-card 
                border-l border-border
                h-screen
                overflow-hidden
                w-full
                "
            >
                {activeView?.type == "display.asset" && (
                    <AssetInspector asset={activeView.asset} />
                )}
            </div>
        </Resizable>
    );
}
