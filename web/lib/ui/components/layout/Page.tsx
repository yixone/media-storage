import type React from "react";

import { buildClassname, type VariantProps } from "@lib/ui/utils/classname";

/**
 * Page Layout options
 */
const pageLayoutVariants = {
    centered: "flex items-center justify-center",
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
            className={buildClassname(pageLayoutVariants[variant], className)}
            {...props}
        />
    );
}

export { PageLayout, pageLayoutVariants };
