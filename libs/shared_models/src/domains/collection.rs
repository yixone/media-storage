use chrono::{DateTime, Utc};

id_type!(CollectionId as uuid::Uuid);

#[derive(Debug, Clone)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct Collection {
    pub id: CollectionId,

    pub created_at: DateTime<Utc>,

    pub title: String,
    pub description: Option<String>,
}

patch_model! {
    CollectionPatchData {
        title: String,
        description: Option<String>
    }
}
