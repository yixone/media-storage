macro_rules! patch_model {
    (
        $( #[$meta: meta] )*
        $model_name: ident { $( $f_id: ident: $f_ty: ty ),* }
    ) => {
        $( #[$meta] )*
        #[derive(Debug, Default)]
        pub struct $model_name {
            $(pub $f_id: $crate::patch::PatchField<$f_ty> ),*
        }
    };
}
