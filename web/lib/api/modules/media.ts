import { AbstractModule } from "./module";

/**
 * Media API Module
 */
export class ApiMediaModule extends AbstractModule {
    /**
     * Returns a link to get a media file by id
     */
    getMediaUrl(id: string) {
        return this.client.root + "/v0/media/" + id;
    }
}
