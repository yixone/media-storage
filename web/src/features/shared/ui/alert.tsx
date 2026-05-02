import { cn, type VariantProps } from "@/utils";

const alertVariants = {
    variant: {
        default: "bg-card text-card-foreground border-border",
        destructive: "bg-card text-destructive border-destructive",
    },
};

type AlertProps = React.ComponentProps<"div"> &
    VariantProps<typeof alertVariants>;

export function Alert({
    className,
    variant = "default",
    ...props
}: AlertProps) {
    return (
        <div
            className={cn(
                "group/alert relative grid w-full gap-0.5 rounded-lg border px-3 py-2 text-left text-sm",
                alertVariants.variant[variant],
                className
            )}
            {...props}
        />
    );
}

export function AlertTitle({
    className,
    ...props
}: React.ComponentProps<"div">) {
    return (
        <div
            data-slot="alert-title"
            className={cn("text-lg font-medium [&_a]:underline", className)}
            {...props}
        />
    );
}

export function AlertDescription({
    className,
    ...props
}: React.ComponentProps<"div">) {
    return (
        <div
            className={cn(
                "text-sm text-balance text-muted-foreground md:text-pretty [&_a]:underline [&_a]:underline-offset-3 [&_p:not(:last-child)]:mb-4",
                className
            )}
            {...props}
        />
    );
}
