import type React from "react";

import { buildClassname, type VariantProps } from "@lib/ui/utils/classname";

/**
 * Page Layout options
 */
const pageLayoutVariants = {
    variant: {
        centered: "flex items-center justify-center",
        scrollable: "flex items-start justify-center overflow-y-scroll",
    },
};

/**
 * Layout styles for the page
 */
function PageLayout({
    variant = "centered",
    className,
    ...props
}: VariantProps<typeof pageLayoutVariants> & React.ComponentProps<"div">) {
    return (
        <div
            className={buildClassname(
                pageLayoutVariants.variant[variant],
                className
            )}
            {...props}
        />
    );
}

export { PageLayout, pageLayoutVariants };
