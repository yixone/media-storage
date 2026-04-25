import type { Asset } from "../types";
import { AbstractModule } from "./module";

/**
 * Asset API Module
 */
export class ApiAssetModule extends AbstractModule {
    /**
     * Returns a list of assets with pagination
     */
    async getList(cursor: number = 0, offset: number = 50): Promise<Asset[]> {
        const list: Asset[] = await this.client.get("v0/assets", {
            cursor: cursor.toString(),
            offset: offset.toString(),
        });
        return list;
    }
}
