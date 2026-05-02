import { cn } from "@/utils";

type CardProps = React.ComponentProps<"div">;

export function Card({ className, ...props }: CardProps) {
    return (
        <div
            className={cn(
                "flex flex-col gap-4 overflow-hidden rounded-xl bg-card text-sm text-card-foreground ring-1 ring-foreground/10",
                className
            )}
            {...props}
        />
    );
}
