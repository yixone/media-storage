import { Outlet } from "react-router";

export function ViewLayout() {
    return (
        <div className="size-full relative">
            <Outlet />
        </div>
    );
}
