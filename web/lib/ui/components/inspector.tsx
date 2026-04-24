import { useApi } from "@lib/api/context";
import type { Asset } from "@lib/api/types";
import React, { useState } from "react";

const INSPECTOR_WIDTH = "30rem";

type InspectorContextProps = {
    /**
     * Is the inspector open?
     */
    inspectorOpen: boolean;

    /**
     * Toggles the inspector state
     */
    toggleInspector: () => void;

    /**
     * Displays asset information in the inspector
     */
    displayAsset: (asset?: Asset) => void;

    /**
     * The asset selected for display in the inspector
     */
    selectedAsset?: Asset;
};

/**
 * Context for the inspector
 */
const InspectorContext = React.createContext<InspectorContextProps | null>(
    null
);

/**
 * Hook for interacting with the inspector context
 */
function useInspector() {
    const ctx = React.useContext(InspectorContext);
    if (!ctx) {
        throw new Error(
            "useInspector() hook must be used within an InspectorProvider"
        );
    }

    return ctx;
}

/**
 * Inspector for displaying information
 */
function InspectorProvider({ children }: React.ComponentProps<"div">) {
    const [open, setOpen] = useState(false);
    const [asset, setAsset] = useState<Asset | undefined>(undefined);

    const contextValue = React.useMemo<InspectorContextProps>(
        () => ({
            inspectorOpen: open,
            toggleInspector: () => {
                setOpen((s) => !s);
            },
            displayAsset: (a) => {
                if (!open) {
                    setOpen(true);
                }

                setAsset(a);
            },
            selectedAsset: asset,
        }),
        [open, asset]
    );

    return (
        <InspectorContext.Provider value={contextValue}>
            {children}

            <Inspector />
        </InspectorContext.Provider>
    );
}

function Inspector() {
    const { inspectorOpen, selectedAsset } = useInspector();
    const { mediaApi } = useApi();

    return (
        <div
            className="
                bg-card 
                border-l border-border
                h-screen
                transition-[width]
                static
                overflow-hidden
                "
            style={{ width: inspectorOpen ? INSPECTOR_WIDTH : 0 }}
        >
            {selectedAsset && (
                <div
                    className="
                        flex flex-col
                        "
                >
                    <img src={mediaApi.getMediaUrl(selectedAsset.media.id)} />
                    <h2>{selectedAsset.title}</h2>
                </div>
            )}
        </div>
    );
}

export { InspectorContext, useInspector, InspectorProvider };
