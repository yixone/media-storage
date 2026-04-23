import { buildClassname } from "./utils";

/**
 * Badge style options
 */
const badgeVariants = {
    default: "bg-primary text-primary-foreground [a&]:hover:bg-primary/90",
    secondary:
        "bg-secondary text-secondary-foreground [a&]:hover:bg-secondary/90",
    destructive: "bg-destructive text-white [a&]:hover:bg-destructive/90",
    ghost: "[a&]:hover:bg-accent [a&]:hover:text-accent-foreground",
};

/**
 * Badge UI component
 */
function Badge({
    className,
    variant = "default",
    ...props
}: React.ComponentProps<"span"> & { variant?: keyof typeof badgeVariants }) {
    return (
        <span
            className={buildClassname(
                `
                flex items-center justify-center gap-1 overflow-hidden
                rounded-full
                border border-transparent
                px-2 py-0.5 w-fit
                text-xs font-medium whitespace-nowrap
                `,
                badgeVariants[variant],
                className
            )}
            {...props}
        />
    );
}

export { Badge, badgeVariants };
