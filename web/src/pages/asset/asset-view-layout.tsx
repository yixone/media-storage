import { cn } from "@/utils/classname";

export function AssetViewLayout({
    className,
    ...props
}: React.ComponentProps<"div">) {
    return (
        <div
            className={cn("flex flex-col md:flex-row", className)}
            {...props}
        />
    );
}

export function AssetViewContent({
    className,
    children,
    ...props
}: React.ComponentProps<"div">) {
    return (
        <div
            className={cn(
                "md:flex-3 w-full max-h-2/3 md:max-h-full bg-background md:bg-linear-to-b md:from-muted md:to-foreground/10",
                className
            )}
            {...props}
        >
            {children}
        </div>
    );
}

type MediaContainerProps = {
    aspectRatio: number;
} & React.ComponentProps<"div">;

export function AssetViewMediaContainer({
    className,
    aspectRatio,
    ...props
}: MediaContainerProps) {
    return (
        <div
            className={cn(
                "w-full md:w-9/10 max-h-full h-auto flex justify-center",
                className
            )}
            style={{ aspectRatio }}
            {...props}
        />
    );
}

export function AssetViewDetails({
    className,
    ...props
}: React.ComponentProps<"div">) {
    return (
        <div
            className={cn(
                "not-md:border-t border-border md:flex-1 flex flex-col",
                className
            )}
            {...props}
        />
    );
}
