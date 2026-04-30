import React from "react";

import type { ApiClient } from "./client";

import { ApiAssetModule } from "@/features/project/assets/api";
import { ApiMediaModule } from "@/features/project/media/api";

type ApiContextProps = {
    assetApi: ApiAssetModule;
    mediaApi: ApiMediaModule;
};

export const ApiContext = React.createContext<ApiContextProps | null>(null);

export function useApi() {
    const ctx = React.useContext(ApiContext);
    if (!ctx) {
        throw new Error("useApi() hook must be used within an ApiProvider");
    }

    return ctx;
}

export function ApiProvider({
    client,
    children,
}: React.ComponentProps<"div"> & { client: ApiClient }) {
    const ctxValue = {
        assetApi: new ApiAssetModule(client),
        mediaApi: new ApiMediaModule(client),
    };

    return (
        <ApiContext.Provider value={ctxValue}>{children}</ApiContext.Provider>
    );
}
