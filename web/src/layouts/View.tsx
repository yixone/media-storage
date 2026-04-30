import { Outlet } from "react-router";

export function ViewLayout() {
    return (
        <div className="grid relative">
            <Outlet />
        </div>
    );
}
