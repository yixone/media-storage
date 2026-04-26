import React, { useRef, useState } from "react";

import { PageLayout } from "@lib/ui/components/layout/Page";

type UploadingMeta = {
    title: string | null;
    caption: string | null;
    source_url: string | null;
};

function UploadPage() {
    const [file, setFile] = useState<File | null>(null);

    async function uploadAsset() {
        // TODO
    }

    return (
        <PageLayout variant="centered" className="h-screen">
            <div className="gap-4 grid grid-cols-1 md:grid-cols-2 grid-rows-2 md:grid-rows-1 size-full">
                <UploadContainer file={file} onFileSelect={setFile} />
            </div>
        </PageLayout>
    );
}

function UploadContainer({
    file,
    onFileSelect,
}: {
    file: File | null;
    onFileSelect: (f: File) => void;
}) {
    const inputRef = useRef<HTMLInputElement | null>(null);

    return (
        <div className="flex items-center justify-center md:justify-end px-2">
            {file && (
                <div className="*:max-h-100 md:*:max-h-200 *:max-w-150 rounded-md overflow-hidden">
                    <img
                        className="size-full"
                        src={URL.createObjectURL(file)}
                    />
                </div>
            )}
            {!file && (
                <div
                    className="rounded-md border-2 border-border bg-border/50 flex items-center justify-center aspect-3/4 relative w-80 focus-visible:outline-2"
                    tabIndex={0}
                    onKeyDown={(e) => {
                        if (e.code === "Enter" || e.code === "Space") {
                            inputRef.current?.click();
                        }
                    }}
                >
                    <div className="absolute size-full flex items-center justify-center pointer-events-none text-lg">
                        Select File
                    </div>
                    <input
                        type="file"
                        ref={inputRef}
                        className="size-full cursor-pointer opacity-0"
                        tabIndex={-1}
                        onChange={(e) => {
                            const files = e.target.files;
                            if (!files) {
                                console.warn(
                                    "UploadPage.MediaInput - NO FILE SELECTED"
                                );
                                return;
                            }
                            const file = files[0];
                            if (!file.type.startsWith("image/")) {
                                console.warn(
                                    "UploadPage.MediaInput - UNSUPPORTED MIMETYPE"
                                );
                                return;
                            }
                            onFileSelect(file);
                        }}
                    />
                </div>
            )}
        </div>
    );
}

export { UploadPage };
