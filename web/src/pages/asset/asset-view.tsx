import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router";

import type { Assets } from "@/api/models";

import { AssetMedia } from "@/features/assets";
import { mediaAspectRatio, humanMediaSize } from "@/features/media";
import {
    ArrowLeftIcon,
    DeleteIcon,
    EditIcon,
} from "@/features/shared/ui/icons";
import { Button, Input, Separator } from "@/features/shared/ui";

import { useApi } from "@/providers";

function useTarget(id?: string) {
    const { assetsApiV1 } = useApi();

    const [asset, setAsset] = useState<Assets.Asset | null>(null);
    const navigate = useNavigate();

    useEffect(() => {
        (async () => {
            if (!id) return;

            try {
                const asset = await assetsApiV1.get(id);
                setAsset(asset);
            } catch (err) {
                navigate("/");
            }
        })();
    }, []);

    return { asset };
}

function useEditMode() {
    const { assetsApiV1 } = useApi();

    const [editMode, setEditMode] = useState(false);

    const [editData, setEditData] = useState<Assets.UpdateAssetRequest | null>(
        null
    );

    const updateEditData = (e: React.ChangeEvent<HTMLInputElement>) => {
        const { name, value } = e.target;

        setEditData((prev) => ({
            ...prev,
            [name]: value,
        }));
    };

    const startEdit = (asset: Assets.Asset) => {
        setEditData({
            title: asset.title,
            caption: asset.caption,
            source_url: asset.source_url,
        });
        setEditMode(true);
    };

    const applyEdit = async (asset: Assets.Asset) => {
        if (!editData) {
            setEditMode(false);
            return;
        }

        await assetsApiV1.update(asset.id, editData);

        location.reload();
    };

    return { editMode, startEdit, applyEdit, editData, updateEditData };
}

export function AssetViewPage() {
    const { id } = useParams();
    const { asset } = useTarget(id);

    const { editMode, startEdit, applyEdit, editData, updateEditData } =
        useEditMode();

    const navigate = useNavigate();

    if (!asset) return null;

    return (
        <div className="flex flex-col md:flex-row w-full h-screen">
            <div className="md:flex-3 w-full max-h-2/3 md:max-h-full bg-background md:bg-linear-to-b md:from-muted md:to-foreground/10 p-0 md:p-4 flex items-center justify-center relative">
                <div className="absolute top-0 left-0 p-2 md:p-4 z-4">
                    <Button
                        variant="outline"
                        size="icon"
                        className="size-10 md:size-11 p-1 md:p-1.5"
                        onClick={() => {
                            navigate(-1);
                        }}
                    >
                        <ArrowLeftIcon className="text-foreground w-full" />
                    </Button>
                </div>
                <div
                    className="w-full md:w-9/10 max-h-full h-auto flex justify-center"
                    style={{ aspectRatio: mediaAspectRatio(asset.media) }}
                >
                    <div className="box-border">
                        <AssetMedia
                            className="size-full md:border border-border/75 overflow-hidden md:rounded-md"
                            media={asset.media}
                        />
                    </div>
                </div>
            </div>
            <div className="not-md:border-t border-border md:flex-1 flex flex-col p-4 gap-1 not-md:flex-col-reverse">
                <div className="size-full flex flex-col gap-2">
                    <div className="flex flex-col gap-1">
                        {!editMode && (
                            <>
                                <h2 className="text-3xl md:text-2xl w-full whitespace-normal wrap-anywhere font-medium">
                                    {asset.title}
                                </h2>
                                {asset.source_url && (
                                    <a
                                        className="decoration-1 underline decoration-foreground/50"
                                        href={asset.source_url}
                                    >
                                        {asset.source_url}
                                    </a>
                                )}
                            </>
                        )}
                        {editMode && (
                            <>
                                <Input
                                    value={editData?.title ?? ""}
                                    className="enabled:text-3xl enabled:md:text-2xl font-medium"
                                    onChange={updateEditData}
                                    name="title"
                                />
                                <Input
                                    value={editData?.source_url ?? ""}
                                    onChange={updateEditData}
                                    name="source_url"
                                />
                            </>
                        )}
                        <p className="text-foreground/80">
                            {new Date(asset.created_at).toLocaleString()}
                        </p>
                    </div>
                    <Separator />

                    <div className="w-full flex">
                        <div className="w-full flex justify-center items-center">
                            <h2>{asset.media.content_type}</h2>
                        </div>

                        <Separator orientation="vertical" className="w-0.75" />

                        <div className="w-full flex justify-center items-center">
                            <h2>{humanMediaSize(asset.media.blob_size)}</h2>
                        </div>

                        <Separator orientation="vertical" className="w-0.75" />

                        <div className="w-full flex justify-center items-center">
                            {asset.media.width} x {asset.media.height}
                        </div>
                    </div>
                </div>

                <div className="size-full flex items-end gap-1 *:h-9">
                    <Button
                        className="w-full flex items-center"
                        variant={editMode ? "default" : "outline"}
                        onClick={() => {
                            editMode ? applyEdit(asset) : startEdit(asset);
                        }}
                    >
                        {!editMode && (
                            <>
                                <EditIcon
                                    fill="transparent"
                                    className="aspect-square h-1/2"
                                />
                                Edit
                            </>
                        )}
                        {editMode && <>Apply</>}
                    </Button>
                    {!editMode && (
                        <Button
                            className="w-full flex items-center enabled:hover:bg-destructive/70 enabled:hover:text-background"
                            variant="outline"
                        >
                            <>
                                <DeleteIcon className="aspect-square h-1/2" />
                                Delete
                            </>
                        </Button>
                    )}
                </div>
            </div>
        </div>
    );
}
