import { useLogger } from "@/utils";

const log = useLogger("[API_CLIENT]");

type QueryParams = Record<string, string> | URLSearchParams | string[][];

type RequestOptions = {
    json?: unknown;
    body?: BodyInit;

    headers?: HeadersInit;
    params?: Record<string, string>;
    method?: "GET" | "POST" | "PUT" | "PATCH" | "DELETE";
};

export class ApiError extends Error {
    status: number;

    constructor(status: number, message: string) {
        super(message);
        this.status = status;
    }
}

export class ApiClient {
    root: string;

    public constructor(root: string) {
        this.root = root;
    }

    private buildUrl(base: string, path: string, query?: QueryParams) {
        const baseUrl = base + "/" + path;
        if (!query || Object.keys(query).length === 0) return baseUrl;

        return baseUrl + "?" + new URLSearchParams(query).toString();
    }

    private buildRequestInit(opts: RequestOptions): RequestInit {
        if (opts.json) {
            return {
                method: opts.method,
                headers: opts.headers,
                body: JSON.stringify(opts.json),
            };
        }

        return {
            method: opts.method,
            headers: opts.headers,
            body: opts.body,
        };
    }

    public async request<T>(path: string, opts?: RequestOptions) {
        const url = this.buildUrl(this.root, path, opts?.params);
        const init = opts ? this.buildRequestInit(opts) : {};

        try {
            const res = await fetch(url, init);
            if (!res.ok) {
                const text = await res.text();
                throw new ApiError(res.status, text);
            }

            const item: T = await res.json();
            return item;
        } catch (err) {
            log.error(url, err);
            throw err;
        }
    }

    // DEPRECATED: use the `client.request(path, opts)` instead
    async performRequest<T>(url: string, params?: RequestInit): Promise<T> {
        try {
            const res = await fetch(`${this.root}/${url}`, params);
            const item: T = await res.json();

            return item;
        } catch (err) {
            log.error(err);
            throw err;
        }
    }
}
