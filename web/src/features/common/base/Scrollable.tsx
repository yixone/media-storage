import type React from "react";

import { buildClassname } from "../../../ui/utils/classname";

export function Scrollable({
    className,
    style,
    ...props
}: React.ComponentProps<"div">) {
    return (
        <div
            className={buildClassname("overflow-y-scroll", className)}
            style={{ scrollbarWidth: "thin", ...style }}
            {...props}
        />
    );
}
