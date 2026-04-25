import type React from "react";

import { buildClassname } from "@lib/ui/utils/classname";

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
    variant,
    className,
    ...props
}: { variant: keyof typeof pageLayoutVariants } & React.ComponentProps<"div">) {
    return (
        <div
            className={buildClassname(pageLayoutVariants[variant], className)}
            {...props}
        />
    );
}

export { PageLayout, pageLayoutVariants };
