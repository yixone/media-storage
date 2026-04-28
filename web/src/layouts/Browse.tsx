import { Outlet } from "react-router";

import { InspectorProvider } from "../providers";
import { Inspector } from "../components/ui/Inspector";

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
