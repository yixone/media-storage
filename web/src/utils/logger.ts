export function useLogger(namespace: string) {
    return new Logger(namespace);
}

export class Logger {
    namespace: string;

    constructor(namespace: string) {
        this.namespace = namespace;
    }

    public log(...args: any[]) {
        console.log(this.namespace, ...args);
    }

    public error(...args: any[]) {
        console.error(this.namespace, ...args);
    }
}
