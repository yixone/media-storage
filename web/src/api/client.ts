/**
 * Client for interacting with the API
 */
export class ApiClient {
    root: string;

    constructor(root: string) {
        this.root = root;
    }

    /**
     * Performs a `GET` request to the API
     */
    async get<T>(url: string, query?: Record<string, string>): Promise<T> {
        return this.performRequest(
            url + "?" + (query && new URLSearchParams(query).toString())
        );
    }

    /**
     * Executes an API request with the specified parameters
     */
    async performRequest<T>(url: string, params?: RequestInit): Promise<T> {
        try {
            const res = await fetch(`${this.root}/${url}`, params);
            const item: T = await res.json();

            return item;
        } catch (err) {
            console.error(err);

            throw err;
        }
    }
}
