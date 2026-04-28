import { Outlet } from "react-router";

export function ViewLayout() {
    return (
        <div className="flex items-center justify-center">
            <Outlet />
        </div>
    );
}
