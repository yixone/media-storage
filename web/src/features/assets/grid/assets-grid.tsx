import { useState } from "react";

import type { Assets } from "@/api/models";
import type { AssetsListEvents, AssetsListLayout } from "../list";
import { useResizeObserver } from "@/features/shared/utils";
import { AssetsGridItem } from "./assets-grid-item";

function useGridLayout() {
    const [colsCount, setColsCount] = useState(2);
    const [layoutReady, setLayoutReady] = useState(false);

    const calcColsCount = (rootWidth: number) => {
        const cols = Math.max(Math.floor(rootWidth / 230), 2);
        return cols;
    };

    const { targetRef } = useResizeObserver((e) => {
        const newCount = calcColsCount(e[0].contentRect.width);
        setColsCount(newCount);

        if (!layoutReady) {
            setLayoutReady(true);
        }
    });

    return { colsCount, layoutReady, targetRef };
}

export class AssetsGrid implements AssetsListLayout {
    render(
        list: Assets.Asset[],
        selectedId: string | null,
        events: AssetsListEvents
    ) {
        const { colsCount, layoutReady, targetRef } = useGridLayout();

        return (
            <div
                className="grid gap-0.5 relative p-2"
                ref={targetRef}
                style={{
                    gridTemplateColumns: `repeat(${colsCount}, minmax(0, 1fr))`,
                }}
            >
                {layoutReady &&
                    list
                        .filter((a) => a.media.state === "Ready")
                        .map((a) => (
                            <AssetsGridItem
                                asset={a}
                                key={a.id}
                                selected={a.id === selectedId}
                                onSelect={events.onSelectAsset}
                                onOpen={events.onOpenAsset}
                            />
                        ))}
            </div>
        );
    }
}
