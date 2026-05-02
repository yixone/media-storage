import { AbstractModule } from "../../../api/module";
import type { Asset, CreateAssetData } from "./models";

/**
 * Asset API Module
 */
export class ApiAssetModule extends AbstractModule {
    /**
     * Returns a list of assets with pagination
     */
    async getList(cursor: number = 0, limit: number = 50): Promise<Asset[]> {
        const list: Asset[] = await this.client.request("v0/assets", {
            params: {
                cursor: cursor.toString(),
                limit: limit.toString(),
            },
        });
        return list;
    }

    /**
     * Returns an Asset by ID
     */
    async get(id: string): Promise<Asset> {
        let asset: Asset = await this.client.request(`v0/assets/${id}`);
        return asset;
    }

    /**
     * Loads the specified asset into the API
     */
    async upload(data: CreateAssetData) {
        const multipartForm = new FormData();
        multipartForm.append("file", data.attachment);
        if (data.title) {
            multipartForm.append("title", data.title);
        }
        if (data.caption) {
            multipartForm.append("caption", data.caption);
        }
        if (data.source_url) {
            multipartForm.append("source", data.source_url);
        }

        await this.client.request("v0/assets", {
            method: "POST",
            body: multipartForm,
        });
    }
}
