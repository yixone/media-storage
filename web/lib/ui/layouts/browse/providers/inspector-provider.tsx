import React, { useState } from "react";

type InspectorContextProps = {
    display: (node: React.ReactNode) => void;
    node: React.ReactNode | null;
};

const InspectorContext = React.createContext<InspectorContextProps | null>(
    null
);

export function useInspector() {
    const ctx = React.useContext(InspectorContext);
    if (!ctx) {
        throw new Error(
            "useInspector() hook must be used within an InspectorProvider"
        );
    }

    return ctx;
}

export function InspectorProvider({ children }: React.ComponentProps<"div">) {
    const [inspectorNode, setInspectorNode] = useState<React.ReactNode | null>(
        null
    );

    const contextValue = React.useMemo<InspectorContextProps>(
        () => ({
            display: (n) => {
                setInspectorNode(n);
            },
            node: inspectorNode,
        }),
        [inspectorNode]
    );

    return (
        <InspectorContext.Provider value={contextValue}>
            {children}
        </InspectorContext.Provider>
    );
}
