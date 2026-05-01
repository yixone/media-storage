import { useEffect, useRef } from "react";

export function useResizeObserver(callback: ResizeObserverCallback) {
    const targetRef = useRef<HTMLDivElement | null>(null);
    useEffect(() => {
        if (!targetRef.current) return;

        const observer = new ResizeObserver(callback);
        observer.observe(targetRef.current);

        return () => observer.disconnect();
    }, []);

    return { targetRef };
}

export function useIntersectionObserver(
    callback: IntersectionObserverCallback,
    opts?: IntersectionObserverInit
) {
    const targetRef = useRef<HTMLDivElement | null>(null);
    useEffect(() => {
        if (!targetRef.current) return;

        const observer = new IntersectionObserver(callback, opts);
        observer.observe(targetRef.current);

        return () => observer.disconnect();
    }, []);

    return { targetRef };
}
