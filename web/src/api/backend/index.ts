import type { AbstractModule } from "../abstract-module";
import type { ApiClient } from "../client";
import { ApiAssetsV0, ApiMediaV0 } from "./v0";

type ModuleConstructor = new (client: ApiClient) => AbstractModule;

export const MODULES_REGISTRY = {
    assetsApiV0: ApiAssetsV0,
    mediaApiV0: ApiMediaV0,
} as const satisfies Record<string, ModuleConstructor>;

export type ModuleInstances<Reg extends Record<string, ModuleConstructor>> = {
    [K in keyof Reg]: InstanceType<Reg[K]>;
};

export function constructApiMods<Reg extends Record<string, ModuleConstructor>>(
    reg: Reg,
    client: ApiClient
) {
    const mods: Record<string, AbstractModule> = {};

    for (const [modId, ModuleConstructor] of Object.entries(reg)) {
        const module = new ModuleConstructor(client);
        mods[modId] = module;
    }

    return mods as ModuleInstances<typeof reg>;
}
