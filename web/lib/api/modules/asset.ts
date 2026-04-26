import type { Asset, CreateAssetData } from "../types";
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

        await this.client.performRequest("v0/assets", {
            method: "POST",
            body: multipartForm,
        });
    }
}
