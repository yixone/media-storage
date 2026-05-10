import { AbstractModule } from "@/api";
import type { Media } from "@/api/models";

export class ApiMediaV1 extends AbstractModule {
    public getMediaUrl(id: string, format: Media.MediaFormat) {
        return this.client.root + "/v1/media/" + id + "?format=" + format;
    }
}
