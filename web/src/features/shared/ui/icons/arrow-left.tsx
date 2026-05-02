import type { IconProps } from "./icon-props";

export function ArrowLeftIcon({ ...props }: IconProps) {
    return (
        <svg viewBox="0 0 24 24" {...props}>
            <path
                d="M14 7L9 12"
                stroke="currentColor"
                strokeWidth="2"
                strokeLinecap="round"
                strokeLinejoin="round"
            />
            <path
                d="M9 12L14 17"
                stroke="currentColor"
                strokeWidth="2"
                strokeLinecap="round"
                strokeLinejoin="round"
            />
        </svg>
    );
}
