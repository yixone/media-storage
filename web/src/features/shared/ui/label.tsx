import { cn } from "@/utils";

type LabelProps = React.ComponentProps<"label">;

export function Label({ className, ...props }: LabelProps) {
    return (
        <label
            className={cn(
                `
                flex items-center
                gap-2
                leading-none
                font-medium
                select-none
                `,
                className
            )}
            {...props}
        />
    );
}
