import React, { useState } from "react";

import type { Asset } from "@/features/project/assets/models";

export type InspectorView = {
    type: "display.asset";
    asset: Asset;
};

export type InspectorContextProps = {
    viewStack: InspectorView[];
    activeView: InspectorView | undefined;

    addView: (v: InspectorView) => void;
    popView: () => InspectorView | undefined;
};

export const InspectorContext =
    React.createContext<InspectorContextProps | null>(null);

export function useInspector() {
    const ctx = React.useContext(InspectorContext);
    if (!ctx) {
        throw new Error(
            "useInspector() hook must be used within an InspectorProvider"
        );
    }
    return ctx;
}

export function InspectorProvider({ children }: { children: React.ReactNode }) {
    const [viewStackArr, setViewStackArr] = useState<InspectorView[]>([]);
    const activeView =
        viewStackArr.length > 0
            ? viewStackArr[viewStackArr.length - 1]
            : undefined;

    function addView(view: InspectorView) {
        if (activeView?.type === view.type) {
            setViewStackArr((p) => {
                const slice = p.slice(0, viewStackArr.length - 1);
                return [...slice, view];
            });
            return;
        }
        setViewStackArr((p) => [...p, view]);
    }

    function popView() {
        const v = activeView;
        setViewStackArr((p) => p.filter((i) => i !== v));
        return v;
    }

    const contextValue = React.useMemo<InspectorContextProps>(
        () => ({
            viewStack: viewStackArr,
            activeView,
            addView,
            popView,
        }),
        [viewStackArr, activeView]
    );

    return (
        <InspectorContext.Provider value={contextValue}>
            {children}
        </InspectorContext.Provider>
    );
}
