import { Outlet } from "react-router";

export function MinimalLayout() {
    return (
        <div className="size-full relative">
            <Outlet />
        </div>
    );
}
