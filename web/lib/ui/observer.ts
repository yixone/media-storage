import { useEffect, useRef } from "react";

/**
 * Creates a ResizeObserver
 */
function useResizeObserver(callback: ResizeObserverCallback) {
    const targetRef = useRef<HTMLDivElement | null>(null);
    useEffect(() => {
        if (!targetRef.current) return;

        const observer = new ResizeObserver(callback);
        observer.observe(targetRef.current);

        return () => observer.disconnect();
    }, []);

    return { targetRef };
}

export { useResizeObserver };
