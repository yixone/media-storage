import { Scrollable, useIntersectionObserver } from "@/features/shared/utils";

type AssetsListScrollProps = {
    children?: React.ReactNode;
    useEndTrigger: boolean;

    onEndReached: () => void;
};

function useScrollTrigger(triggerCallback: () => void) {
    const { targetRef } = useIntersectionObserver(
        (e) => {
            const entry = e[0];
            if (!entry.isIntersecting) return;
            triggerCallback();
        },
        {
            threshold: 0.25,
            rootMargin: "0px",
        }
    );
    return { targetRef };
}

export function AssetsListScroll({
    children,
    useEndTrigger,
    onEndReached,
}: AssetsListScrollProps) {
    const { targetRef } = useScrollTrigger(onEndReached);

    return (
        <Scrollable className="h-screen w-full">
            <div className="relative">
                {children}

                {useEndTrigger && (
                    <div
                        ref={targetRef}
                        className="absolute h-80 md:h-60 w-full bottom-0 pointer-events-none"
                    />
                )}
            </div>
        </Scrollable>
    );
}
