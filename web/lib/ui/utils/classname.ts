/**
 * Assembles a ClassName from `string?` parts
 */
export function buildClassname(...items: (string | undefined)[]) {
    return items.join(" ");
}

export type VariantProps<T> = { variant?: keyof T };
