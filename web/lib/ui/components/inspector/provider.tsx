import React, { useEffect, useState } from "react";

import type { InspectorView } from "./view";

type InspectorContextProps = {
    stack: InspectorView[];

    push: (v: InspectorView) => void;
    pop: () => InspectorView | undefined;
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
    const [viewStack, setViewStack] = useState<InspectorView[]>([]);

    function pop() {
        const v = viewStack[viewStack.length - 1];
        setViewStack((p) => p.filter((i) => i !== v));
        return v;
    }

    function replace(view: InspectorView) {
        setViewStack((p) => {
            const slice = p.slice(0, viewStack.length - 1);
            return [...slice, view];
        });
    }

    useEffect(() => {
        console.log(viewStack);
    }, [viewStack]);

    const contextValue = React.useMemo<InspectorContextProps>(
        () => ({
            stack: viewStack,

            push: (v) => {
                if (
                    viewStack.length > 0 &&
                    viewStack[viewStack.length - 1].type === v.type
                ) {
                    replace(v);
                    return;
                }
                setViewStack((p) => [...p, v]);
            },
            pop: pop,
        }),
        [viewStack]
    );

    return (
        <InspectorContext.Provider value={contextValue}>
            {children}
        </InspectorContext.Provider>
    );
}
