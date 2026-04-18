macro_rules! id_type {
    (
        $(#[$meta: meta])*
        $id: ident as $id_ty: ty
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash, sqlx::Type, serde::Serialize, serde::Deserialize)]
        #[sqlx(transparent)]
        pub struct $id($id_ty);
    };
}
