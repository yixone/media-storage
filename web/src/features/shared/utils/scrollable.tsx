import { cn } from "@/utils";

type ScrollableProps = React.ComponentProps<"div">;

export function Scrollable({ className, style, ...props }: ScrollableProps) {
    return (
        <div
            className={cn("overflow-y-scroll", className)}
            style={{ scrollbarWidth: "thin", ...style }}
            {...props}
        />
    );
}
