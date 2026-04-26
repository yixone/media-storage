import type React from "react";

import { buildClassname, type VariantProps } from "@lib/ui/utils/classname";

export const buttonVariants = {
    variant: {
        default:
            "bg-primary text-primary-foreground enabled:hover:bg-primary/85",
    },
    size: {
        default: "h-8 gap-1.5 px-2.5",
    },
};

export function Button({
    variant = "default",
    size = "default",
    className,
    ...props
}: VariantProps<typeof buttonVariants> & React.ComponentProps<"button">) {
    return (
        <button
            className={buildClassname(
                `
                flex items-center justify-center
                rounded-lg
                border border-transparent outline-none
                text-sm font-medium whitespace-nowrap select-none
                focus-visible:border-ring focus-visible:ring-2 focus-visible:ring-ring
                disabled:opacity-50 disabled:cursor-not-allowed
                cursor-pointer
                `,
                buttonVariants.variant[variant],
                buttonVariants.size[size],
                className
            )}
            {...props}
        />
    );
}
