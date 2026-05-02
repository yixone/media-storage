import type { ApiClient } from "./client";

export abstract class AbstractModule {
    protected readonly client: ApiClient;

    public constructor(client: ApiClient) {
        this.client = client;
    }
}
