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
