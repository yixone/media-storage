import { useState } from "react";

import { useResizeObserver } from "@lib/ui/observer";

export function Grid({
    children,
    columnWidth = 250,
}: {
    children?: React.ReactNode;
    columnWidth?: number;
}) {
    const [colsCount, setColsCount] = useState(2);
    const [gridReady, setGridReady] = useState(false);

    const calcColsCount = (rootWidth: number) => {
        const cols = Math.max(Math.floor(rootWidth / columnWidth), 2);
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
