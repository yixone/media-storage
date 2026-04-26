import React, { useState } from "react";

import { PageLayout } from "@lib/ui/components/layout/Page";

function UploadPage() {
    const [file, setFile] = useState<File | null>(null);

    async function uploadAsset() {
        // TODO
    }

    return (
        <PageLayout variant="centered" className="h-screen">
            <div
                className="
                gap-4 
                grid grid-cols-1 md:grid-cols-2 grid-rows-2 md:grid-rows-1 
                size-full
                "
            >
                <UploadContainer file={file} setFile={setFile} />
            </div>
        </PageLayout>
    );
}

function UploadContainer({
    file,
    setFile,
}: {
    file: File | null;
    setFile: (f: File) => void;
}) {
    function onFileSelect(e: React.ChangeEvent<HTMLInputElement>) {
        const files = e.target.files;
        if (!files) {
            console.warn("UploadPage.MediaInput - NO FILE SELECTED");
            return;
        }
        const file = files[0];

        // TEMP
        if (!file.type.startsWith("image/")) {
            console.warn("UploadPage.MediaInput - UNSUPPORTED MIMETYPE");
            return;
        }

        setFile(files[0]);
    }

    return (
        <div className="flex items-center justify-center md:justify-end">
            <div className="rounded-md overflow-hidden border-2 border-border">
                {file && (
                    <div className="size-full *:max-h-100 md:*:max-h-200 *:max-w-150">
                        <img
                            className="size-full"
                            src={URL.createObjectURL(file)}
                        />
                    </div>
                )}
                {!file && (
                    <div className="bg-border/75 flex items-center justify-center aspect-3/4 relative">
                        <div className="absolute size-full flex items-center justify-center pointer-events-none text-lg">
                            Select File
                        </div>
                        <input
                            type="file"
                            className="size-full cursor-pointer opacity-0"
                            onChange={onFileSelect}
                        />
                    </div>
                )}
            </div>
        </div>
    );
}

export { UploadPage };
