import { buildClassname } from "@lib/ui/utils/classname";

export function AppLayout({
    className,
    ...props
}: React.ComponentProps<"div">) {
    return (
        <div
            className={buildClassname("flex flex-row w-full", className)}
            {...props}
        />
    );
}
