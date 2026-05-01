import type React from "react";

import { buildClassname, type VariantProps } from "./utils/classname";

export const buttonVariants = {
    variant: {
        default:
            "bg-primary text-primary-foreground enabled:hover:bg-primary/85",
        outline:
            "border-border bg-background hover:bg-muted hover:text-foreground",
        secondary:
            "bg-secondary text-secondary-foreground hover:bg-secondary/80",
    },
    size: {
        default: "h-8 gap-1.5 px-2.5",
        icon: "size-9",
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
                buttonVariants.variant[variant],
                buttonVariants.size[size],
                `
                flex items-center justify-center
                rounded-lg
                border outline-none
                text-sm font-medium whitespace-nowrap select-none
                focus-visible:border-ring focus-visible:ring-2 focus-visible:ring-ring
                disabled:opacity-50 disabled:cursor-not-allowed
                cursor-pointer
                `,
                className
            )}
            {...props}
        />
    );
}
