import { Outlet } from "react-router";

import { Inspector, InspectorProvider } from "@/features/common/inspector";

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
