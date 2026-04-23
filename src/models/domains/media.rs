use chrono::{DateTime, Utc};

use crate::{error::Result, files::storage::types::StorageKey, models::types::UpdateField};

id_type! {
    /// Media Id
    MediaId as String
}

impl From<MediaId> for StorageKey {
    fn from(id: MediaId) -> Self {
        StorageKey { inner: id.0 }
    }
}

impl From<StorageKey> for MediaId {
    fn from(id: StorageKey) -> Self {
        MediaId(id.inner)
    }
}

/// Media Domain
#[derive(Debug, PartialEq, sqlx::FromRow)]
pub struct Media {
    /// Hash-based media Id
    pub id: MediaId,

    /// The state of media
    pub state: MediaState,

    /// Media creation time
    pub created_at: DateTime<Utc>,

    /// Media Size In Bytes
    pub size: i64,

    /// Content-Type
    pub mimetype: String,

    /// Media Width
    pub width: Option<u16>,
    /// Media Height
    pub height: Option<u16>,

    /// Media Accent Color
    pub color: Option<String>,
}

/// Media State
#[derive(Debug, Clone, Copy, PartialEq, sqlx::Type, serde::Serialize)]
#[sqlx(rename_all = "lowercase")]
pub enum MediaState {
    /// Media is awaiting processing
    Pending,
    /// The media is being processed.
    Processing,
    /// The media is ready to deliver
    Ready,
    /// Media processing failed
    Failed,
}

/// Media Update DTO
#[derive(Debug, Default)]
pub struct MediaUpdateData {
    /// New state of media
    pub state: UpdateField<MediaState>,

    /// New Media Width
    pub width: UpdateField<Option<u16>>,
    /// New Media Height
    pub height: UpdateField<Option<u16>>,

    /// New Media Accent Color
    pub color: UpdateField<Option<String>>,
}

/// Trait for [`Media`] domain repository
#[async_trait::async_trait]
pub trait MediaRepository {
    /// Inserts a [`Media`] into the database
    async fn insert_media(&self, media: &Media) -> Result<()>;

    /// Deletes an [`Media`] from the database by [`MediaId`]
    async fn delete_media(&self, id: &MediaId) -> Result<()>;

    /// Updates [`Media`] fields in the database
    /// according to [`MediaUpdateData`]
    async fn update_media(&self, id: &MediaId, data: &MediaUpdateData) -> Result<bool>;

    /// Returns a [`Media`] from the database by [`MediaId`]
    async fn get_media(&self, id: &MediaId) -> Result<Option<Media>> {
        self.get_media_by_ids(&[id])
            .await
            .map(|m| m.into_iter().next())
    }

    /// Returns a set of [`Media`] by the list of [`MediaId`]
    async fn get_media_by_ids(&self, ids: &[&MediaId]) -> Result<Vec<Media>>;

    /// Returns a list of [`Media`] pending processing
    async fn get_pending_media(&self, cursor: u32, limit: u32) -> Result<Vec<Media>>;
}
