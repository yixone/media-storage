import type React from "react";

import { buildClassname } from "./utils/classname";

export function Label({ className, ...props }: React.ComponentProps<"label">) {
    return (
        <label
            className={buildClassname(
                `
                flex items-center
                gap-2
                leading-none
                font-medium
                select-none
                `,
                className
            )}
            {...props}
        />
    );
}
