import { useState } from "react";

export interface InspectorView {
    viewType: string;
    render(): React.ReactNode;
}

export function useInspectorStack() {
    const [viewStack, setViewStack] = useState<InspectorView[]>([]);
    const activeView =
        viewStack.length > 0 ? viewStack[viewStack.length - 1] : undefined;

    function addView(view: InspectorView) {
        if (activeView?.viewType === view.viewType) {
            setViewStack((p) => {
                const slice = p.slice(0, viewStack.length - 1);
                return [...slice, view];
            });
            return;
        }
        setViewStack((p) => [...p, view]);
    }

    function popView() {
        const v = activeView;
        setViewStack((p) => p.filter((i) => i !== v));

        return v;
    }

    return { viewStack, activeView, addView, popView };
}
