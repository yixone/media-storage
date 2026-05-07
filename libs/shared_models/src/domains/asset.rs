use chrono::{DateTime, Utc};

use crate::domains::MediaId;

id_type!(AssetId as i64);

#[derive(Debug, Clone)]
pub struct Asset {
    pub id: AssetId,
    pub media: MediaId,

    pub created_at: DateTime<Utc>,

    pub title: Option<String>,
    pub caption: Option<String>,
}

patch_model! {
    AssetPatchData {
        title: Option<String>,
        caption: Option<String>
    }
}
