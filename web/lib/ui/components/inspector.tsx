import type { Asset } from "@lib/api/types";
import React, { useState } from "react";
import { DateDisplay } from "./date";
import { MediaDisplay } from "./features/media/MediaDisplay";

const INSPECTOR_WIDTH = "30rem";

type InspectorContextProps = {
    toggleInspector: () => void;
    inspectorOpen: boolean;

    displayAsset: (asset?: Asset) => void;
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
    const [open, setOpen] = useState(true);
    const [selectedAsset, setSelectedAsset] = useState<Asset | undefined>(
        undefined
    );

    const contextValue = React.useMemo<InspectorContextProps>(
        () => ({
            toggleInspector: () => {
                setOpen((s) => !s);
            },
            inspectorOpen: open,

            displayAsset: (a) => {
                if (!open) setOpen(true);
                setSelectedAsset(a);
            },
            selectedAsset,
        }),
        [open, selectedAsset]
    );

    return (
        <InspectorContext.Provider value={contextValue}>
            {children}
        </InspectorContext.Provider>
    );
}

/**
 * Inspector root object
 */
function Inspector({ children }: React.ComponentProps<"div">) {
    const { inspectorOpen } = useInspector();

    return (
        <div
            className="
                bg-card 
                border-l border-border
                h-screen
                overflow-hidden
                "
            style={{ width: inspectorOpen ? INSPECTOR_WIDTH : 0 }}
        >
            {children}
        </div>
    );
}

/**
 * An inspector component for displaying information about the selected asset
 */
function AssetInspector() {
    const { selectedAsset } = useInspector();

    if (!selectedAsset) return;

    return (
        <div
            className="
            flex flex-col
            p-4
            gap-2
            "
        >
            <div className="flex items-center justify-center">
                <MediaDisplay
                    media={selectedAsset.media}
                    className="overflow-hidden border border-border/65 rounded-[0.5rem] max-h-100 min-h-5"
                />
            </div>

            <h2 className="text-xl w-full whitespace-normal wrap-anywhere">
                {selectedAsset.title}
            </h2>
            {selectedAsset.source_url && (
                <a
                    href={selectedAsset.source_url}
                    className="text-foreground/65"
                >
                    <b>{"Source: "}</b>
                    <i className="decoration-1 underline">
                        {selectedAsset.source_url}
                    </i>
                </a>
            )}
            <h2 className="text-foreground/65">
                <b>{"Created: "}</b>
                <DateDisplay date={new Date(selectedAsset.created_at)} />
            </h2>
            <h2 className="text-foreground/65">
                <b>{"Size: "}</b>
                {(selectedAsset.media.size / 1024 / 1024).toFixed(2)} Mb
            </h2>
        </div>
    );
}

export {
    InspectorContext,
    useInspector,
    InspectorProvider,
    Inspector,
    AssetInspector,
};
