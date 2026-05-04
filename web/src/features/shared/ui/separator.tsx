import { cn, type VariantProps } from "@/utils";

const separatorVatiants = {
    orientation: {
        horizontal: "h-px w-full",
        vertical: "w-0.5 self-stretch",
    },
};

type ButtonProps = {
    className?: string;
} & VariantProps<typeof separatorVatiants>;

export function Separator({
    className,
    orientation = "horizontal",
}: ButtonProps) {
    return (
        <div
            className={cn(
                "bg-border",
                separatorVatiants.orientation[orientation],
                className
            )}
        />
    );
}
