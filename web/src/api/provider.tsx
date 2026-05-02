import React from "react";

import {
    MODULES_REGISTRY,
    constructApiMods,
    type ModuleInstances,
} from "./backend";

import type { ApiClient } from "./client";

type ApiProviderProps = { client: ApiClient; children?: React.ReactNode };
type ApiContextProps = ModuleInstances<typeof MODULES_REGISTRY>;

export const ApiContext = React.createContext<ApiContextProps | null>(null);

export function useApi() {
    const ctx = React.useContext(ApiContext);
    if (!ctx) {
        throw new Error("useApi() hook must be used within an ApiProvider");
    }
    return ctx;
}

export function ApiProvider({ client, children }: ApiProviderProps) {
    const apiModules = constructApiMods(MODULES_REGISTRY, client);
    const contextData: ApiContextProps = {
        ...apiModules,
    };

    return (
        <ApiContext.Provider value={contextData}>
            {children}
        </ApiContext.Provider>
    );
}
