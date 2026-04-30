import React, { useState } from "react";

import { useResizeObserver } from "@/features/common/observer";
import { buildClassname } from "@/ui/utils/classname";

type AssetsGridLayoutProps = { children: React.ReactNode };

export function AssetsGridLayout({ children }: AssetsGridLayoutProps) {
    const [colsCount, setColsCount] = useState(2);
    const [gridReady, setGridReady] = useState(false);

    const calcColsCount = (rootWidth: number) => {
        const cols = Math.max(Math.floor(rootWidth / 230), 2);
        return cols;
    };

    const { targetRef } = useResizeObserver((e) => {
        const newCount = calcColsCount(e[0].contentRect.width);
        setColsCount(newCount);

        if (!gridReady) {
            setGridReady(true);
        }
    });

    return (
        <div
            className="grid gap-1 overflow-hidden w-full"
            ref={targetRef}
            style={{
                gridTemplateColumns: `repeat(${colsCount}, minmax(0, 1fr))`,
            }}
        >
            {gridReady && children}
        </div>
    );
}

type GridAssetLayoutProps = React.ComponentProps<"div">;
type GridAssetMediaLayoutProps = React.ComponentProps<"div">;

export function GridAssetLayout({
    children,
    className,
    ...props
}: GridAssetLayoutProps) {
    return (
        <div
            className={buildClassname(
                `
                group/grid-asset 
                cursor-pointer 
                bg-transparent hover:bg-foreground/4 data-[selected=true]:bg-foreground/8 
                rounded-md p-1 
                flex flex-col 
                outline-none`,
                className
            )}
            {...props}
        >
            {children}
        </div>
    );
}

export function GridAssetMediaLayout({
    children,
    className,
    ...props
}: GridAssetMediaLayoutProps) {
    return (
        <div
            className={buildClassname(
                "aspect-square flex justify-center items-center",
                className
            )}
            {...props}
        >
            {children}
        </div>
    );
}
