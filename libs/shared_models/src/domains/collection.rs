use chrono::{DateTime, Utc};

id_type!(CollectionId as uuid::Uuid);

#[derive(Debug, Clone)]
pub struct Collection {
    pub id: CollectionId,

    pub created_at: DateTime<Utc>,

    pub title: String,
    pub description: Option<String>,
}
