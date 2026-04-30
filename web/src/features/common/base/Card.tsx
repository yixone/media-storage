import type React from "react";

import { buildClassname } from "../../../ui/utils/classname";

/**
 * Card for displaying content
 */
function Card({ className, ...props }: React.ComponentProps<"div">) {
    return (
        <div
            className={buildClassname(
                "flex flex-col gap-4 overflow-hidden rounded-xl bg-card text-sm text-card-foreground ring-1 ring-foreground/10",
                className
            )}
            {...props}
        />
    );
}

export { Card };
