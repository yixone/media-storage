import React from "react";
import { AssetApi } from "./Asset";
import { MediaApi } from "./Media";
import type { ApiClient } from "./client";

type ApiContextProps = {
    assetApi: AssetApi;
    mediaApi: MediaApi;
};

export const ApiContext = React.createContext<ApiContextProps | null>(null);

export function useApi() {
    const ctx = React.useContext(ApiContext);
    if (!ctx) {
        throw new Error("useApi must be used within a ApiProvider");
    }

    return ctx;
}

export function ApiProvider({
    client,
    children,
}: React.ComponentProps<"div"> & { client: ApiClient }) {
    const ctxValue = {
        assetApi: new AssetApi(client),
        mediaApi: new MediaApi(client),
    };

    return (
        <ApiContext.Provider value={ctxValue}>{children}</ApiContext.Provider>
    );
}
