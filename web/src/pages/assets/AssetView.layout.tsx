import { buildClassname } from "@/ui/utils/classname";

export function AssetViewLayout({
    className,
    ...props
}: React.ComponentProps<"div">) {
    return (
        <div
            className={buildClassname("flex flex-col md:flex-row", className)}
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
            className={buildClassname(
                "md:flex-3 w-full max-h-2/3 md:max-h-full bg-background md:bg-muted",
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
            className={buildClassname(
                "w-full max-h-full h-auto flex justify-center",
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
            className={buildClassname(
                "not-md:border-t border-border md:flex-1 flex flex-col",
                className
            )}
            {...props}
        />
    );
}
