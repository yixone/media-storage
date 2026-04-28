import { buildClassname } from "@lib/ui/classname";
import React from "react";
import { useState } from "react";

export function Resizable({
    children,
    handleSide = "l",
    defaultWidth = 25,
    className,
}: {
    children: React.ReactNode;
    handleSide?: "l" | "r";
    defaultWidth?: number;
    className?: string;
}) {
    const [resizableWidth, setResizableWidth] = useState(defaultWidth);
    const [resizing, setResizing] = useState(false);

    const handleResizeStart = () => {
        if (resizing) return;

        console.log("RESIZE - START");

        document.addEventListener("mousemove", handleResize);
        document.addEventListener("mouseup", handleResizeEnd);
    };

    const handleResize = (e: MouseEvent) => {
        const coef = handleSide == "l" ? -1 : 1;
        setResizableWidth((w) => w + e.movementX * (coef / 10));
    };

    const handleResizeEnd = () => {
        console.log("RESIZE - END");
        setResizing(false);

        document.removeEventListener("mousemove", handleResize);
        document.removeEventListener("mouseup", handleResizeEnd);
    };

    return (
        <div
            className={buildClassname("h-full relative", className)}
            style={{
                width: `${resizableWidth}rem`,
            }}
        >
            <div
                className={buildClassname(
                    "cursor-col-resize w-0.75 absolute h-full bg-ring/20 select-none z-2 hover:bg-ring pointer-events-auto",
                    handleSide == "l" ? "left-0" : "right-0"
                )}
                onMouseDown={handleResizeStart}
            />

            {children}
        </div>
    );
}
