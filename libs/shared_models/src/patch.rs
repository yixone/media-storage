#[derive(Debug, Default)]
pub enum PatchField<T> {
    Update(T),
    #[default]
    Ignore,
}

macro_rules! patch_model {
    (
        $model_name: ident { $( $f_id: ident: $f_ty: ty ),* }
    ) => {
        #[derive(Default)]
        pub struct $model_name {
            $(pub $f_id: $crate::patch::PatchField<$f_ty> ),*
        }
    };
}
