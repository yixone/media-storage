import { cn } from "@/utils";

type SkeletonProps = React.ComponentProps<"div">;

export function Skeleton({ className, ...props }: SkeletonProps) {
    return (
        <div
            className={cn(
                "animate-pulse rounded-md bg-foreground/10",
                className
            )}
            {...props}
        />
    );
}
