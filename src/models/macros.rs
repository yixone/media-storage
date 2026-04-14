macro_rules! id_type {
    (
        $(#[$meta: meta])*
        $id: ident as $id_ty: ty
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Hash)]
        pub struct $id {
            pub inner: $id_ty,
        }
    };
}
