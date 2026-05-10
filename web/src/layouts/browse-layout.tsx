import { Outlet } from "react-router";

export function BrowseLayout() {
    return (
        <div className="flex w-full relative">
            <Outlet />
        </div>
    );
}
