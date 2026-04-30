import type { ApiClient } from "./client";

/**
 * Abstract class for API Modules
 */
export abstract class AbstractModule {
    protected client: ApiClient;

    public constructor(client: ApiClient) {
        this.client = client;
    }
}
