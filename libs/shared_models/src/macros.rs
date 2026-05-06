macro_rules! id_type {
    (
        $( #[$meta: meta] )*
        $id: ident as $id_ty: ty
    ) => {
        $( #[$meta] )*
        #[derive(Debug, Clone, PartialEq)]
        pub struct $id(pub $id_ty);
    };
}
