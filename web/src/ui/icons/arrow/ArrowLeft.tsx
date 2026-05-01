import type { IconProps } from "../props";

export function ArrowLeftIcon({ ...props }: IconProps) {
    return (
        <svg viewBox="0 0 24 24" fill="none" {...props}>
            <path
                d="M14 7L9 12"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
            />
            <path
                d="M9 12L14 17"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
            />
        </svg>
    );
}
