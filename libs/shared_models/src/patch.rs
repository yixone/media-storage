pub enum PatchField<T> {
    Update(T),
    Ignore,
}

macro_rules! patch_model {
    (
        $model_name: ident { $( $f_id: ident: $f_ty: ty ),* }
    ) => {
        pub struct $model_name {
            $(pub $f_id: $crate::patch::PatchField<$f_ty> ),*
        }
    };
}
