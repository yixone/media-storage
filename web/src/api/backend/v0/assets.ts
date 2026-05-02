import { AbstractModule } from "@/api/abstract-mod";
import type { Assets } from "@/api/models";

export class ApiAssetsV0 extends AbstractModule {
    public async getList(
        cursor: number = 0,
        limit: number = 50
    ): Promise<Assets.Asset[]> {
        const list: Assets.Asset[] = await this.client.request("v0/assets", {
            params: {
                cursor: cursor.toString(),
                limit: limit.toString(),
            },
        });
        return list;
    }

    public async get(id: string): Promise<Assets.Asset> {
        let asset: Assets.Asset = await this.client.request(`v0/assets/${id}`);
        return asset;
    }

    public async upload(data: Assets.CreateAssetRequest) {
        const multipartForm = new FormData();
        multipartForm.append("file", data.attachment);

        if (data.title) multipartForm.append("title", data.title);
        if (data.caption) multipartForm.append("caption", data.caption);
        if (data.source_url) multipartForm.append("source", data.source_url);

        await this.client.request("v0/assets", {
            method: "POST",
            body: multipartForm,
        });
    }
}
