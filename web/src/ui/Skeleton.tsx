import { buildClassname } from "./utils/classname";

type SkeletonProps = React.ComponentProps<"div">;

export function Skeleton({ className, ...props }: SkeletonProps) {
    return (
        <div
            className={buildClassname(
                "animate-pulse rounded-md bg-foreground/10",
                className
            )}
            {...props}
        />
    );
}
