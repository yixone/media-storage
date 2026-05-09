use std::sync::Arc;

use ms_content_store::key::StorageKey;
use ms_database::{pagination::Pagination, traits::MediaRepoExt};
use ms_media::image::MediaImage;
use ms_shared_models::{
    domains::{Media, MediaId, MediaPatchData, MediaStatus},
    patch::PatchField,
};
use tokio::sync::mpsc::{Receiver, Sender, channel};

use crate::{di::DataContext, error::AppResult};

#[derive(Debug)]
pub enum MediaWorkerTask {
    NewMedia { id: MediaId },
}

impl MediaWorkerTask {
    pub async fn send(self, sender: &Sender<MediaWorkerTask>) {
        if let Err(e) = sender.send(self).await {
            tracing::error!(err = ?e, "Failed to send MediaWorkerTask");
        }
    }
}

async fn update_media_status_helper(
    media: &Media,
    status: MediaStatus,
    ctx: &DataContext,
) -> AppResult<bool> {
    let res = ctx
        .db
        .patch_media(
            &media.id,
            &MediaPatchData {
                status: PatchField::Update(status),
                ..Default::default()
            },
        )
        .await?;
    Ok(res)
}

async fn process_image_command(img_media: &Media, ctx: &DataContext) -> AppResult<()> {
    let key = StorageKey::from_str_unchecked(&img_media.id.0);
    let reader = ctx.store.get(key).await?;

    let img = MediaImage::from_reader(reader).await?;

    let (w, h) = img.get_dimension();
    let color = format!("#{}", hex::encode(img.get_color()));

    ctx.db
        .patch_media(
            &img_media.id,
            &MediaPatchData {
                width: PatchField::Update(Some(w as u16)),
                height: PatchField::Update(Some(h as u16)),
                color: PatchField::Update(Some(color)),
                status: PatchField::Update(MediaStatus::Ready),
            },
        )
        .await?;

    Ok(())
}

async fn process_media_command(media: &Media, ctx: &DataContext) -> AppResult<()> {
    if !update_media_status_helper(media, MediaStatus::Processing, ctx).await? {
        return Ok(());
    }
    tracing::info!(media = ?media.id, "media_worker.processing_media");

    if let Err(e) = process_image_command(media, ctx).await {
        update_media_status_helper(media, MediaStatus::Failed, ctx).await?;
        return Err(e);
    }

    Ok(())
}

pub struct MediaWorker {
    ctx: Arc<DataContext>,

    tx: Sender<MediaWorkerTask>,
    rx: Receiver<MediaWorkerTask>,
}

impl MediaWorker {
    pub fn new(ctx: Arc<DataContext>, channel_buf_size: usize) -> Self {
        let (tx, rx) = channel(channel_buf_size);
        Self { ctx, tx, rx }
    }

    pub fn sender(&self) -> Sender<MediaWorkerTask> {
        self.tx.clone()
    }

    async fn runtime(mut self) -> AppResult<()> {
        let pending = self
            .ctx
            .db
            .get_pending_media(Pagination::new(50, 0))
            .await?;

        for m in pending {
            process_media_command(&m, &self.ctx).await?;
        }

        while let Some(task) = self.rx.recv().await {
            match task {
                MediaWorkerTask::NewMedia { id } => {
                    tracing::info!(id = ?id, "media_worker.new_media_task_received");
                    let Some(media) = self.ctx.db.get_media(&id).await? else {
                        continue;
                    };
                    if media.status != MediaStatus::Pending {
                        continue;
                    }
                    process_media_command(&media, &self.ctx).await?;
                }
            }
        }
        Ok(())
    }

    pub async fn spawn(self) -> Sender<MediaWorkerTask> {
        let tx = self.tx.clone();
        tokio::spawn(async move {
            let rt = self.runtime().await;
            if let Err(e) = rt {
                tracing::error!(err = ?e, "media_worker.runtime_error");
            }
        });
        tx
    }
}
