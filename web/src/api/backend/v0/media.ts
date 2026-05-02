import { AbstractModule } from "@/api/abstract-mod";

export class ApiMediaV0 extends AbstractModule {
    public getMediaUrl(id: string) {
        return this.client.root + "/v0/media/" + id;
    }
}
