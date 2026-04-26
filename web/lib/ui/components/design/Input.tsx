import { buildClassname } from "@lib/ui/utils/classname";
import type React from "react";

export function Input({
    className,
    type,
    ...props
}: React.ComponentProps<"input">) {
    return (
        <input
            className={buildClassname(
                `
                h-8 w-full min-w-0 
                rounded-lg border border-input bg-transparent
                px-2.5 py-1.5
                text-base placeholder:text-muted-foreground
                outline-none
                focus-visible:border-ring focus-visible:ring-2 focus-visible:ring-ring
                disabled:cursor-not-allowed disabled:bg-input/50 disabled:opacity-50
                invalid:border-destructive invalid:ring-2 invalid:ring-destructive
                md:text-sm
                `,
                className
            )}
            {...props}
        />
    );
}
