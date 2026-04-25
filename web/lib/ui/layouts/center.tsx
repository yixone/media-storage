import type React from "react";
import { buildClassname } from "../components/utils";

function CenterLayout({ className, ...props }: React.ComponentProps<"div">) {
    return (
        <div
            className={buildClassname(
                "w-screen h-screen flex items-center justify-center",
                className
            )}
            {...props}
        />
    );
}

export { CenterLayout };
