import { Outlet } from "react-router";

import { InspectorProvider } from "@/providers";
import { Inspector } from "@/features/inspector";

export function BrowseLayout() {
    return (
        <InspectorProvider>
            <div className="flex w-full relative">
                <Outlet />

                <Inspector />
            </div>
        </InspectorProvider>
    );
}
