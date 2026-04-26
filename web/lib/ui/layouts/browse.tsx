import { Outlet } from "react-router";

import { Inspector, InspectorProvider } from "@lib/ui/components/inspector";

/**
 * Basic Layout with Sidebar and Inspector
 */
export function BrowseLayout() {
    return (
        <InspectorProvider>
            <div className="flex w-full">
                <Outlet />

                <Inspector />
            </div>
        </InspectorProvider>
    );
}
