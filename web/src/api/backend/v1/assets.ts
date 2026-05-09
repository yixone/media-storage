import { AbstractModule } from "@/api";
import type { Assets } from "@/api/models";

export class ApiAssetsV1 extends AbstractModule {
    public async getList(
        offset: number = 0,
        limit: number = 50
    ): Promise<Assets.Asset[]> {
        const list: Assets.Asset[] = await this.client.request("v1/assets", {
            params: {
                offset: offset.toString(),
                limit: limit.toString(),
            },
        });
        return list;
    }

    public async get(id: string): Promise<Assets.Asset> {
        let asset: Assets.Asset = await this.client.request(`v1/assets/${id}`);
        return asset;
    }

    public async upload(data: Assets.CreateAssetRequest) {
        const multipartForm = new FormData();
        multipartForm.append("file", data.attachment);

        if (data.title) multipartForm.append("title", data.title);
        if (data.caption) multipartForm.append("caption", data.caption);
        if (data.source_url)
            multipartForm.append("source_url", data.source_url);

        await this.client.request("v1/assets", {
            method: "POST",
            body: multipartForm,
        });
    }
}
