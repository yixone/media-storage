export type VariantProps<T> = { [K in keyof T]?: keyof T[K] };

export function cn(...items: (string | undefined)[]) {
    return items.join(" ");
}
