import React, { useEffect, useRef, useState } from "react";
import { useNavigate } from "react-router";

import type { CreateAssetData } from "@lib/api/types";
import { useApi } from "@lib/api/context";

import { Button, Input, Label } from "@lib/ui/design";

// TODO:
// Rewrite this bad code

type UploadData = Pick<CreateAssetData, "caption" | "source_url" | "title">;

function UploadPage() {
    const [file, setFile] = useState<File | null>(null);

    const uploadDataObj: UploadData = {
        caption: null,
        source_url: null,
        title: null,
    };
    const [uploadData, setUploadData] = useState(uploadDataObj);

    const [uploading, setUploading] = useState(false);

    const { assetApi } = useApi();
    const navigate = useNavigate();

    function handleUploadDataChange(e: React.ChangeEvent<HTMLInputElement>) {
        const { name, value } = e.target;
        setUploadData((prev) => ({
            ...prev,
            [name]: value,
        }));
    }

    async function uploadAsset() {
        setUploading(true);

        if (!file) {
            setUploading(false);
            return;
        }

        const DTO: CreateAssetData = {
            attachment: file,
            ...uploadData,
        };

        await assetApi.upload(DTO);
        navigate("/");
    }

    return (
        <div className="py-6 gap-4 grid grid-cols-1 md:grid-cols-2 grid-rows-2 md:grid-rows-1 w-full">
            <UploadContainer
                file={file}
                onFileSelect={(f) => {
                    setUploadData((prev) => ({
                        ...prev,
                        title: f.name,
                    }));
                    setFile(f);
                }}
            />
            <MetadataContainer
                data={uploadData}
                onDataChange={handleUploadDataChange}
                enabled={uploading ? false : file !== null}
                onUpload={uploadAsset}
            />
        </div>
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

    function handleInput(e: React.ChangeEvent<HTMLInputElement>) {
        const files = e.target.files;
        if (!files) return;
        selectFile(files[0]);
    }

    function handlePaste(e: ClipboardEvent) {
        if (!e.clipboardData || e.clipboardData.files.length == 0) return;
        const file = e.clipboardData.files[0];
        selectFile(file);
    }

    function selectFile(f: File) {
        if (file !== null) return;
        if (!f.type.startsWith("image/")) {
            console.warn("UploadPage.MediaInput - UNSUPPORTED MIMETYPE");
            return;
        }
        onFileSelect(f);
    }

    useEffect(() => {
        window.addEventListener("paste", handlePaste);
        return () => window.removeEventListener("paste", handlePaste);
    }, []);

    return (
        <div className="flex items-center justify-center md:justify-end">
            {file && (
                <div className="*:max-h-100 md:*:max-h-200 *:max-w-150 rounded-md overflow-hidden border border-border/50">
                    <img
                        className="size-full"
                        src={URL.createObjectURL(file)}
                    />
                </div>
            )}
            {!file && (
                <div
                    className="rounded-md border-2 border-border bg-input/50 flex items-center justify-center aspect-3/4 relative w-80 focus-visible:ring-3 focus-visible:ring-ring outline-none"
                    tabIndex={1}
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
                        onChange={handleInput}
                    />
                </div>
            )}
        </div>
    );
}

function MetadataContainer({
    data,
    onDataChange,
    enabled,
    onUpload,
}: {
    data: UploadData;
    onDataChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
    enabled: boolean;
    onUpload: () => void;
}) {
    return (
        <div className="flex flex-col items-center md:items-start justify-start gap-2 w-full md:min-w-75 md:w-1/2 *:flex *:flex-col *:gap-1">
            <div>
                <Label htmlFor="asset_title">Title</Label>
                <Input
                    id="asset_title"
                    name="title"
                    placeholder="Title"
                    value={data.title ?? ""}
                    onChange={onDataChange}
                    disabled={!enabled}
                />
            </div>
            <div>
                <Label htmlFor="asset_caption">Caption</Label>
                <Input
                    id="asset_caption"
                    name="caption"
                    placeholder="Caption"
                    value={data.caption ?? ""}
                    onChange={onDataChange}
                    disabled={!enabled}
                />
            </div>
            <div>
                <Label htmlFor="asset_source_url">Source URL</Label>
                <Input
                    id="asset_source_url"
                    name="source_url"
                    placeholder="Source URL"
                    value={data.source_url ?? ""}
                    onChange={onDataChange}
                    disabled={!enabled}
                />
            </div>

            <Button
                disabled={!enabled}
                className="mt-2"
                onClick={() => onUpload()}
            >
                Upload
            </Button>
        </div>
    );
}

export { UploadPage };
