import { AbstractModule } from "@/api";

export class ApiMediaV1 extends AbstractModule {
    public getMediaUrl(id: string) {
        return this.client.root + "/v1/media/" + id;
    }
}
