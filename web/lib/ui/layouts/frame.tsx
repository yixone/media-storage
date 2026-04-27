import { Outlet } from "react-router";

/**
 * Single Frame Layout
 */
export function FrameLayout() {
    return (
        <div className="flex items-center justify-center">
            <Outlet />
        </div>
    );
}
