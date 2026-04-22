use tokio::sync::mpsc::{Receiver, Sender, channel};
use tracing::info;

use crate::{
    create_error,
    db::providers::Database,
    error::Result,
    files::storage::Storage,
    media::image::MediaImage,
    models::{
        domains::{Media, MediaId, MediaState, MediaUpdateData},
        types::UpdateField,
    },
};

/// [`MediaWorker`] Tasks
pub enum MediaWorkerTask {
    /// New media has been uploaded and needs to be processed
    NewMedia { id: MediaId },
}

impl MediaWorkerTask {
    /// Send a task to the specified channel
    pub async fn send(self, sender: &Sender<MediaWorkerTask>) -> Result<()> {
        sender.send(self).await.map_err(|e| {
            tracing::error!(err = ?e, "media_worker.send_task_error");
            create_error!(InternalError)
        })
    }
}

/// Background Media worker
pub struct MediaWorker {
    db: Database,
    store: Storage,

    tx: Sender<MediaWorkerTask>,
    rx: Receiver<MediaWorkerTask>,
}

impl MediaWorker {
    /// Creates a new [`MediaWorker`]
    pub fn new(db: Database, store: Storage) -> Self {
        let (tx, rx) = channel(128);
        Self { db, store, tx, rx }
    }

    /// Returns [`MediaWorker`] sender
    pub fn sender(&self) -> Sender<MediaWorkerTask> {
        self.tx.clone()
    }

    /// Spawns a worker in a separate thread
    /// and returns [`Sender`] for sending
    /// [`MediaWorkerTask`] to the [`MediaWorker`]
    pub async fn spawn(self) {
        tokio::spawn(async move {
            let rt = self.run().await;
            if let Err(e) = rt {
                tracing::error!(err = ?e, "media_worker.service_error");
            }
        });
    }

    // FIXME: This is a "placeholder" code. Rewrite it later.
    async fn run(mut self) -> Result<()> {
        let pending = self.db.get_pending_media(0, 50).await?;
        for m in pending {
            self.process_media(&m).await?;
        }

        while let Some(task) = self.rx.recv().await {
            match task {
                MediaWorkerTask::NewMedia { id } => {
                    info!(id = ?id, "media_worker.new_media_task_received");
                    let Some(media) = self.db.get_media(&id).await? else {
                        continue;
                    };
                    if media.state != MediaState::Pending {
                        continue;
                    }
                    self.process_media(&media).await?;
                }
            }
        }
        Ok(())
    }

    async fn update_media_state(&self, media: &Media, state: MediaState) -> Result<bool> {
        self.db
            .update_media(
                &media.id,
                &MediaUpdateData {
                    state: UpdateField::Set(state),
                    ..Default::default()
                },
            )
            .await
    }

    async fn process_media(&self, media: &Media) -> Result<()> {
        if !self
            .update_media_state(media, MediaState::Processing)
            .await?
        {
            return Ok(());
        }
        info!(media = ?media.id, "media_worker.processing_media");
        self.process_image(media).await?;
        Ok(())
    }

    async fn process_image(&self, img: &Media) -> Result<()> {
        let key = img.id.clone().into();
        let reader = self.store.get(&key).await?;

        let image = MediaImage::from_reader(reader).await?;
        let (w, h) = image.get_dimension();
        let color = hex::encode(image.get_accent_color());

        self.db
            .update_media(
                &img.id,
                &MediaUpdateData {
                    width: UpdateField::Set(Some(w as u16)),
                    height: UpdateField::Set(Some(h as u16)),
                    color: UpdateField::Set(Some(color)),
                    state: UpdateField::Set(MediaState::Ready),
                },
            )
            .await?;
        Ok(())
    }
}
