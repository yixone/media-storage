import type { Assets } from "@/api/models";
import React, { useRef, useState } from "react";
import { useApi } from "./api-provider";

type AssetsListContextProps = {
    assets: Assets.Asset[];
    nextBatch: () => Promise<void>;
    resetList: () => void;

    initList: (listId?: string) => void;
};

export const AssetsListContext =
    React.createContext<AssetsListContextProps | null>(null);

export function useAssetsList() {
    const ctx = React.useContext(AssetsListContext);
    if (!ctx) {
        throw new Error(
            "useAssetsList() hook must be used within an AssetsListContext"
        );
    }

    return ctx;
}

type AssetsListProviderProps = {
    listPageSize?: number;
    children?: React.ReactNode;
};

export function AssetsListProvider({
    listPageSize = 25,
    children,
}: AssetsListProviderProps) {
    const { assetsApiV1 } = useApi();

    const [assets, setAssets] = useState<Assets.Asset[]>([]);

    const currentList = useRef<string | undefined>(undefined);

    const offset = useRef(0);
    const assetsOut = useRef(false);
    const listLoading = useRef(false);

    const listScrollY = useRef(0);

    function resetList(listId?: string) {
        currentList.current = listId;
        offset.current = 0;
        assetsOut.current = false;
        listScrollY.current = 0;
        setAssets([]);
    }

    const nextBatch = React.useCallback(async () => {
        if (assetsOut.current || listLoading.current) {
            return;
        }

        listLoading.current = true;

        const data = await assetsApiV1.getList(offset.current, listPageSize);
        setAssets((a) => [...a, ...data]);

        if (data.length < listPageSize) {
            assetsOut.current = true;
        }

        offset.current += data.length;
        listLoading.current = false;
    }, []);

    function initList(listId?: string) {
        if (listId !== currentList.current) {
            resetList(listId);
        }
    }

    const contextValue = React.useMemo(
        () => ({
            assets,
            nextBatch,
            resetList,
            initList,
        }),
        [assets, nextBatch]
    );

    return (
        <AssetsListContext.Provider value={contextValue}>
            {children}
        </AssetsListContext.Provider>
    );
}
