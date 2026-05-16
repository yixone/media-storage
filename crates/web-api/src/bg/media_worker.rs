use std::{sync::Arc, time::Duration};

use asset_shelf_database::traits::MediaRepositoryExt;
use asset_shelf_media::image::{FilterType, ImageFormat, MediaImage};
use asset_shelf_models::{
    domains::{Media, MediaId, MediaPatch, MediaPreviewKey, MediaStatus},
    patch::PatchField,
};
use asset_shelf_result::error::AppResult;
use asset_shelf_storage::StorageKey;
use tokio::sync::mpsc::{Receiver, Sender, channel};

use crate::di::DataContext;

/// Task queue buffer size
const QUEUE_BUFFER_SIZE: usize = 128;
/// Limit per pending media request
const PENDING_MEDIA_FETCH_LIMIT: u32 = 50;
/// Cooldown between worker restart
const RETRY_COOLDOWN_SECS: u64 = 5;

/// Tasks for [`MediaWorker`]
#[derive(Debug)]
pub enum MediaWorkerTask {
    /// Complete media uploading and processing
    FinalizeUploadedMedia { id: MediaId },
}

impl MediaWorkerTask {
    /// Sends a [`MediaWorkerTask`] to the [`MediaWorker`]
    /// and returns `true` if the task was received
    pub async fn send(self, sender: &Sender<MediaWorkerTask>) -> bool {
        if let Err(e) = sender.send(self).await {
            tracing::warn!(error = ?e, "Failed to send media worker task");
            return false;
        }
        true
    }
}

/// Background Worker for media processing
pub struct MediaWorker {
    /// Worker Data Access context
    data: Arc<DataContext>,

    /// Worker tasks sender
    tx: Sender<MediaWorkerTask>,
    /// Receiver for worker tasks
    rx: Receiver<MediaWorkerTask>,
}

impl MediaWorker {
    /// Creates a new [`MediaWorker`]
    pub fn new(data: Arc<DataContext>) -> Self {
        let (tx, rx) = channel(QUEUE_BUFFER_SIZE);
        MediaWorker { data, tx, rx }
    }

    /// Spawns a [`MediaWorker`] and returns a tasks [`Sender`]
    pub fn spawn(mut self) -> Sender<MediaWorkerTask> {
        let tx = self.tx.clone();

        tokio::spawn(async move {
            loop {
                if let Err(e) = self.runtime().await {
                    if e.kind.is_internal() {
                        tracing::error!(err = ?e, "Media worker internal error occurred. Terminating its runtime!");
                        break;
                    } else {
                        tracing::error!(err = ?e, "Media worker runtime error occurred");
                    }
                }
                tokio::time::sleep(Duration::from_secs(RETRY_COOLDOWN_SECS)).await;
            }
        });

        tx
    }

    /// [`MediaWorker`] runtime
    async fn runtime(&mut self) -> AppResult<()> {
        let service = MediaWorkerService { data: &self.data };

        let pending_media = service.get_pending_media(PENDING_MEDIA_FETCH_LIMIT).await?;
        for m in pending_media {
            service.process_media(&m).await?;
        }

        while let Some(task) = self.rx.recv().await {
            match task {
                MediaWorkerTask::FinalizeUploadedMedia { id } => {
                    let Some(media) = self.data.db.get_media(&id).await? else {
                        continue;
                    };
                    if media.status != MediaStatus::Pending {
                        continue;
                    }
                    service.process_media(&media).await?;
                }
            }
        }

        Ok(())
    }
}

/// Service for media processing worker
struct MediaWorkerService<'a> {
    data: &'a DataContext,
}

impl<'a> MediaWorkerService<'a> {
    /// Returns a list of media pending processing.
    async fn get_pending_media(&self, limit: u32) -> AppResult<Vec<Media>> {
        self.data.db.get_pending_media(limit).await
    }

    /// Changes the [`Media`] status
    async fn update_media_status(&self, id: &MediaId, status: MediaStatus) -> AppResult<bool> {
        self.data
            .db
            .update_media(
                id,
                &MediaPatch {
                    status: PatchField::Update(status),
                    ..Default::default()
                },
            )
            .await
    }

    // TODO: Add video processing
    /// Calls [`Media`] processing
    async fn process_media(&self, media: &Media) -> AppResult<()> {
        if !self
            .update_media_status(&media.id, MediaStatus::Processing)
            .await?
        {
            return Ok(());
        }
        tracing::info!(media = ?media.id, "Media worker processes media");

        if let Err(e) = self.process_image(media).await {
            self.update_media_status(&media.id, MediaStatus::Failed)
                .await?;
            return Err(e);
        }

        Ok(())
    }

    /// Processes [`Media`] as an [`MediaImage`]
    async fn process_image(&self, image_media: &Media) -> AppResult<()> {
        let key = StorageKey::from_str_unchecked(&image_media.id.0);
        let reader = self.data.store.get(key).await?;

        let img = MediaImage::from_reader(reader).await?;

        let (w, h) = img.dimension();
        let accent_color = format!("#{}", hex::encode(img.accent_color()));

        let preview_img = img
            .thumbnail(350, FilterType::Triangle)
            .reader(ImageFormat::WebP)?;
        let stored_preview = self.data.store.put(preview_img).await?;
        let preview_key = MediaPreviewKey(stored_preview.key.inner);

        let patch = MediaPatch {
            preview_key: PatchField::Update(preview_key),
            preview_size: PatchField::Update(stored_preview.size as i64),
            accent_color: PatchField::Update(Some(accent_color)),
            width: PatchField::Update(Some(w as u16)),
            height: PatchField::Update(Some(h as u16)),
            status: PatchField::Update(MediaStatus::Ready),
            ..Default::default()
        };
        self.data.db.update_media(&image_media.id, &patch).await?;

        Ok(())
    }
}
