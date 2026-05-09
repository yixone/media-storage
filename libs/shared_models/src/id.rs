macro_rules! id_type {
    (
        $( #[$meta: meta] )*
        $id: ident as $id_ty: ty
    ) => {
        $( #[$meta] )*
        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "sqlx", derive(sqlx::Type), sqlx(transparent))]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct $id(pub $id_ty);
    };
}

pub fn generate_i64_id() -> i64 {
    const TIMESTAMP_EPOCH: i64 = 1735678800000;

    let timestamp = (chrono::Utc::now().timestamp_millis() - TIMESTAMP_EPOCH) as u64;
    let rand = rand::random::<u16>() as u64;

    // [ TIMESTAMP (48) ] | [ RANDOM (16) ]
    ((timestamp << 16) | (rand & 0xFFFF)) as i64
}
