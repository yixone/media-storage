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
    ...props
}: React.ComponentProps<"div">) {
    return (
        <div
            className={buildClassname(
                "md:flex-3 md:h-full bg-background md:bg-muted flex items-center",
                className
            )}
            {...props}
        />
    );
}

type MediaContainerProps = {
    aspectRatio: number;
} & React.ComponentProps<"div">;

export function AssetViewMediaContainer({
    aspectRatio,
    className,
    ...props
}: MediaContainerProps) {
    return (
        <div
            className={buildClassname(
                "max-h-150 w-full md:max-h-full flex justify-center items-center",
                className
            )}
            style={{
                aspectRatio,
            }}
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
            className={buildClassname("md:flex-1 flex flex-col", className)}
            {...props}
        />
    );
}
