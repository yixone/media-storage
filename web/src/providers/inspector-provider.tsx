import React from "react";

import { useInspectorStack, type InspectorView } from "@/features/inspector";

type InspectorContextProps = {
    viewStack: InspectorView[];
    activeView: InspectorView | undefined;

    addView: (v: InspectorView) => void;
    popView: () => InspectorView | undefined;
};

type InspectorProviderProps = { children: React.ReactNode };

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

export function InspectorProvider({ children }: InspectorProviderProps) {
    const { viewStack, activeView, addView, popView } = useInspectorStack();

    const contextData = React.useMemo<InspectorContextProps>(
        () => ({
            viewStack,
            activeView,
            addView,
            popView,
        }),
        [viewStack, activeView]
    );

    return (
        <InspectorContext.Provider value={contextData}>
            {children}
        </InspectorContext.Provider>
    );
}
